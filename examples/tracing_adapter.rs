use sheen::{Level, Logger, SheenLayer};
use tracing_subscriber::prelude::*;

fn main() {
    let logger = Logger::new().level(Level::Trace);
    let layer = SheenLayer::new(logger);

    tracing_subscriber::registry().with(layer).init();

    tracing::trace!("starting up");
    tracing::debug!("loading configuration");
    tracing::info!("server listening on port 3000");
    tracing::warn!("cache is nearly full");
    tracing::error!("failed to connect to database");
}
