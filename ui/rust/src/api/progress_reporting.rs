use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::sync::Arc;

#[derive(Clone)]
#[frb(opaque)]
pub struct DartProgressReporter {
    report_total_sink: Arc<StreamSink<String>>,
    report_progress_sink: Arc<StreamSink<String>>,
    message_sink: Arc<StreamSink<String>>,
    finish_sink: Arc<StreamSink<bool>>,
}

impl ProgressReporter for DartProgressReporter {
    fn start_for_download(&self, total_work: u64) {
        let _ = self.report_total_sink.add(total_work.to_string());
    }

    fn start_without_len(&self) {
        let _ = self.report_total_sink.add("0".to_string());
    }

    fn report_progress(&self, progress: u64) {
        let _ = self.report_progress_sink.add(progress.to_string());
    }

    fn report_message(&self, message: &str) {
        let _ = self.message_sink.add(message.to_string());
    }

    fn finish(&self) {
        let _ = self.finish_sink.add(true);
    }
}

#[frb(sync)]
pub fn create_dart_progress_reporter(
    report_total_sink: StreamSink<String>,
    report_progress_sink: StreamSink<String>,
    message_sink: StreamSink<String>,
    finish_sink: StreamSink<bool>,
) -> DartProgressReporter {
    DartProgressReporter {
        report_total_sink: Arc::new(report_total_sink),
        report_progress_sink: Arc::new(report_progress_sink),
        message_sink: Arc::new(message_sink),
        finish_sink: Arc::new(finish_sink),
    }
}
