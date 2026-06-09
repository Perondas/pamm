use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
#[frb(opaque)]
pub struct DartProgressReporter {
    sink: Arc<StreamSink<String>>,
}

impl ProgressReporter for DartProgressReporter {
    fn start_for_download(&self, total_work: u64) {
        let payload = json!({"type": "total", "total": total_work});
        let _ = self.sink.add(payload.to_string());
    }

    fn start_without_len(&self) {
        // Use total = 0 to indicate unknown length, as previous implementation used "0".
        let payload = json!({"type": "total", "total": 0});
        let _ = self.sink.add(payload.to_string());
    }

    fn report_progress(&self, progress: u64) {
        let payload = json!({"type": "progress", "progress": progress});
        let _ = self.sink.add(payload.to_string());
    }

    fn report_message(&self, message: &str) {
        let payload = json!({"type": "message", "message": message});
        let _ = self.sink.add(payload.to_string());
    }

    fn finish(&self) {
        let payload = json!({"type": "finish", "finished": true});
        let _ = self.sink.add(payload.to_string());
    }
}

#[frb(sync)]
pub fn create_dart_progress_reporter(
    sink: StreamSink<String>,
    // We have to add this stupid dummy sink so that the bridge generator actually works.
    // As otherwise it changes the return time to Strem<String> isntead of the opaque type
    _dummy: StreamSink<()>,
) -> DartProgressReporter {
    DartProgressReporter {
        sink: Arc::new(sink),
    }
}
