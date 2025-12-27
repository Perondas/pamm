pub trait ProgressReporter: Send + Sync + Clone {
    fn start(&self, total_work: u64);
    fn start_without_len(&self);
    fn report_progress(&self, progress: u64);
    fn report_message(&self, message: &str);
    fn finish(&self);
}
