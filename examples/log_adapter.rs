use sheen::{Level, Logger};

fn main() {
    // sheen as the log backend
    Logger::new().level(Level::Trace).init().unwrap();

    // These are log crate macros, not sheen macros
    log::trace!("starting up");
    log::debug!("loading configuration");
    log::info!("server listening on port 3000");
    log::warn!("cache is nearly full");
    log::error!("failed to connect to database");
}
