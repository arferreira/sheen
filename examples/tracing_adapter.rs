use sheen::{Level, Logger, SheenLayer};

fn main() {
    SheenLayer::new(Logger::new().level(Level::Trace)).init();

    tracing::trace!("starting up");
    tracing::debug!("loading configuration");
    tracing::info!("server listening on port 3000");
    tracing::warn!("cache is nearly full");
    tracing::error!("failed to connect to database");
}
