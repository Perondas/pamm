//! Based on the indicatif-log-bridge crate https://crates.io/crates/indicatif-log-bridge

use env_logger::Logger;
use indicatif::ProgressBar;
use log::Log;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct LogWrapper {
    bar: Arc<RwLock<Option<ProgressBar>>>,
    log: Arc<Logger>,
}

impl LogWrapper {
    pub fn new(log: Logger) -> Self {
        Self {
            bar: Arc::default(),
            log: Arc::new(log),
        }
    }

    pub fn try_init(self) -> Result<Self, log::SetLoggerError> {
        use log::LevelFilter::*;
        let levels = [Off, Error, Warn, Info, Debug, Trace];

        for level_filter in levels.iter().rev() {
            let level = if let Some(level) = level_filter.to_level() {
                level
            } else {
                // off is the last level, just do nothing in that case
                continue;
            };
            let meta = log::Metadata::builder().level(level).build();
            if self.enabled(&meta) {
                log::set_max_level(*level_filter);
                break;
            }
        }

        log::set_boxed_logger(Box::new(self.clone()))?;
        Ok(self)
    }

    pub fn set_progress_bar(&self, bar: ProgressBar) {
        let mut lock = self.bar.write().unwrap();
        *lock = Some(bar);
    }

    pub fn clear_progress_bar(&self) {
        let mut lock = self.bar.write().unwrap();
        *lock = None;
    }
}

impl Log for LogWrapper {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.log.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        // do an early check for enabled to not cause unnescesary suspends
        if self.log.enabled(record.metadata()) {
            if let Some(bar) = self.bar.read().unwrap().as_ref() {
                bar.suspend(|| self.log.log(record));
            } else {
                self.log.log(record);
            }
        }
    }

    fn flush(&self) {
        self.log.flush()
    }
}
