use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use log::{Level, Log, Metadata, Record};
use std::str::FromStr;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, LazyLock, Mutex};

static LOGGER: LazyLock<Mutex<Option<FlutterLogger>>> = LazyLock::new(Mutex::default);

#[frb(sync)]
pub fn init_rust_logger(level: String, log_sink: StreamSink<String>) {
    let level = Level::from_str(&level).unwrap();

    {
        if let Some(logger) = LOGGER.lock().unwrap().as_mut() {
            logger.log_sink = Arc::new(log_sink);
            logger.level.store(level as u8, Relaxed);

            return;
        }
    }

    let logger = FlutterLogger::new(level, log_sink);

    log::set_boxed_logger(Box::new(logger.clone())).expect("failed to set logger");
    log::set_max_level(log::LevelFilter::Trace);
    log::logger().log(
        &Record::builder()
            .args(format_args!("Logger initialized with level: {}", level))
            .build(),
    );

    LOGGER.lock().unwrap().replace(logger);
}

#[frb(sync)]
pub fn set_rust_log_level(level: String) {
    let level = Level::from_str(&level).unwrap();

    LOGGER
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .level
        .store(level as u8, Relaxed);
}

#[frb(ignore)]
#[derive(Clone)]
pub struct FlutterLogger {
    level: Arc<AtomicU8>,
    log_sink: Arc<StreamSink<String>>,
}

impl FlutterLogger {
    pub fn new(level: Level, log_sink: StreamSink<String>) -> FlutterLogger {
        Self {
            level: Arc::new(AtomicU8::new(level as u8)),
            log_sink: Arc::new(log_sink),
        }
    }
}

impl Log for FlutterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() as u8 <= self.level.load(Relaxed)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{} - {}", record.level(), record.args());
            self.log_sink
                .add(log_message)
                .expect("Failed to send log message to Flutter");
        }
    }

    fn flush(&self) {
        // No buffering, so nothing to flush
    }
}
