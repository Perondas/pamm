use indicatif::ProgressBar;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct IndicatifProgressReporter {
    progress_bar: Arc<RwLock<Option<ProgressBar>>>,
}

impl IndicatifProgressReporter {
    pub fn new() -> Self {
        Self {
            progress_bar: Arc::new(RwLock::new(None)),
        }
    }
}

impl ProgressReporter for IndicatifProgressReporter {
    fn start(&self, total_work: u64) {
        let progress_bar = ProgressBar::new(total_work);
        let mut pb_lock = self.progress_bar.write().unwrap();
        *pb_lock = Some(progress_bar);
    }

    fn start_without_len(&self) {
        let progress_bar = ProgressBar::no_length();
        let mut pb_lock = self.progress_bar.write().unwrap();
        *pb_lock = Some(progress_bar);
    }

    fn report_progress(&self, progress: u64) {
        if let Some(ref pb) = *self.progress_bar.read().unwrap() {
            pb.inc(progress);
        }
    }

    fn report_message(&self, message: &str) {
        if let Some(ref pb) = *self.progress_bar.read().unwrap() {
            pb.println(message);
        }
    }

    fn finish(&self) {
        // Finish and clear the progress bar
        let mut pb_lock = self.progress_bar.write().unwrap();
        if let Some(ref pb) = *pb_lock {
            pb.finish_and_clear();
        }
        *pb_lock = None;
    }
}
