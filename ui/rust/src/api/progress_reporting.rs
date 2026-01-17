use crate::frb_generated::StreamSink;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::sync::Arc;

#[derive(Clone)]
pub struct DartProgressReporter {
    report_progress_sink: Arc<StreamSink<String>>,
    message_sink: Arc<StreamSink<String>>,
}

impl ProgressReporter for DartProgressReporter {
    fn start_for_download(&self, total_work: u64) {
        // NOOP
    }

    fn start_without_len(&self) {
        //NOOP
    }

    fn report_progress(&self, progress: u64) {
        let _ = self.report_progress_sink.add(progress.to_string());
    }

    fn report_message(&self, message: &str) {
        let _ = self.message_sink.add(message.to_string());
    }

    fn finish(&self) {
        todo!()
    }
}
