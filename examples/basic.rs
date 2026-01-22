use sheen::Level;
use sheen::Logger;

fn main() {
    let logger = Logger::new()
        .level(Level::Trace)
        .timestamp(true)
        .prefix("sheen");

    logger.info("Server started", &[("port", &3000)]);
    logger.debug("Loading config", &[]);
    logger.error("Something broke", &[("code", &500)]);
}
