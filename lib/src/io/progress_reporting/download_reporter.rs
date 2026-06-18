use crate::io::progress_reporting::progress_reporter::ProgressReporter;

pub(crate) trait DownloadReporter: Send + Sync + Clone {
    fn report_progress(&self, progress: u64);
}

impl<T> DownloadReporter for T
where
    T: ProgressReporter,
{
    #[inline]
    fn report_progress(&self, progress: u64) {
        self.report_progress(progress);
    }
}

#[derive(Clone, Default)]
pub struct NoopDownloadReporter;

impl DownloadReporter for NoopDownloadReporter {
    #[inline]
    fn report_progress(&self, _progress: u64) {}
}
