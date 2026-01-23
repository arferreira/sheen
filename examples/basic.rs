use sheen::{Level, LogFmtFormatter, Logger};

fn main() {
    let logger = Logger::new().level(Level::Trace).formatter(LogFmtFormatter);

    logger.info("started", &[("port", &3000)]);
}
