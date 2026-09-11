pub trait ProgressReporter: Send + Sync + Clone {
    fn start_for_download(&self, total_work: u64);
    fn start_without_len(&self);
    fn report_progress(&self, progress: u64);
    fn report_message(&self, message: &str);
    fn finish(&self);
}

#[derive(Clone, Default)]
pub struct NoopProgressReporter;

impl ProgressReporter for NoopProgressReporter {
    fn start_for_download(&self, _total_work: u64) {}
    fn start_without_len(&self) {}
    fn report_progress(&self, _progress: u64) {}
    fn report_message(&self, _message: &str) {}
    fn finish(&self) {}
}
