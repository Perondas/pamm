use indicatif::{ProgressBar, ProgressStyle};
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Clone)]
pub struct IndicatifProgressReporter {
    enabled: bool,
    progress_bar: Arc<RwLock<Option<ProgressBar>>>,
}

impl Default for IndicatifProgressReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl IndicatifProgressReporter {
    pub fn new() -> Self {
        Self {
            enabled: true,
            progress_bar: Arc::new(RwLock::new(None)),
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            progress_bar: Arc::new(RwLock::new(None)),
        }
    }
}

impl ProgressReporter for IndicatifProgressReporter {
    fn start_for_download(&self, total_work: u64) {
        if !self.enabled {
            return;
        }
        let progress_bar = ProgressBar::new(total_work);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {bytes}/{total_bytes}@{bytes_per_sec} ({eta})")
            .unwrap());
        progress_bar.enable_steady_tick(Duration::from_secs(1));
        let mut pb_lock = self.progress_bar.write().unwrap();
        *pb_lock = Some(progress_bar);
    }

    fn start_without_len(&self) {
        if !self.enabled {
            return;
        }
        let progress_bar = ProgressBar::no_length();
        progress_bar.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap());
        progress_bar.enable_steady_tick(Duration::from_secs(1));
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
