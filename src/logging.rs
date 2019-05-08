use log::{Level, Metadata, Record};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = crate::time::monotonic();
            println!(
                "[{:<5}]({:.5}): {}",
                record.level(),
                time.0 as f32 + time.1 as f32 / 1000000000.,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
