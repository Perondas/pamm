use crate::io::progress_reporting::progress_reporter::ProgressReporter;

pub trait ProgressReporterProvider: Send + Sync {
    fn get_progress_reporter(&self) -> Box<impl ProgressReporter>;
    fn get_fixed_length_progress_reporter(&self, total_length: u64) -> Box<impl ProgressReporter>;
}