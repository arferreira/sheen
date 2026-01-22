use crate::Logger;
use std::fmt::Debug;
use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init() {
    let _ = LOGGER.set(Logger::new());
}

pub fn init_with(logger: Logger) {
    let _ = LOGGER.set(logger);
}

pub fn logger() -> &'static Logger {
    LOGGER.get_or_init(Logger::new)
}

pub fn info(message: &str, fields: &[(&str, &dyn Debug)]) {
    logger().info(message, fields);
}
