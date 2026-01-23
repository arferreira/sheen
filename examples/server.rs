use std::{thread, time::Duration};

fn main() {
    sheen::init_with(
        sheen::Logger::new()
            .level(sheen::Level::Trace)
            .prefix("server"),
    );

    // Startup sequence
    sheen::info!("initializing server");
    sleep(300);

    sheen::debug!("loading configuration", file = "config.toml");
    sleep(200);

    sheen::trace!("parsed environment variables", count = 12);
    sleep(150);

    sheen::info!("connecting to database", host = "localhost", port = 5432);
    sleep(400);

    sheen::debug!("connection pool ready", size = 10);
    sleep(200);

    sheen::info!("server started", port = 3000, env = "development");
    sleep(600);

    // Simulated requests
    sheen::info!("request received", method = "GET", path = "/api/users");
    sleep(150);
    sheen::trace!("authenticating request", token = "eyJhbG...");
    sleep(100);
    sheen::debug!("user authenticated", user_id = 42);
    sleep(200);
    sheen::info!("response sent", status = 200, duration_ms = 45);
    sleep(500);

    sheen::info!("request received", method = "POST", path = "/api/orders");
    sleep(150);
    sheen::debug!("validating payload", content_type = "application/json");
    sleep(100);
    sheen::trace!("payload validated", bytes = 1024);
    sleep(200);
    sheen::info!("response sent", status = 201, duration_ms = 123);
    sleep(500);

    sheen::info!("request received", method = "GET", path = "/api/products");
    sleep(100);
    sheen::warn!("cache miss", key = "products:all");
    sleep(150);
    sheen::debug!("fetching from database", query = "SELECT * FROM products");
    sleep(300);
    sheen::info!("response sent", status = 200, duration_ms = 287);
    sleep(500);

    sheen::info!(
        "request received",
        method = "DELETE",
        path = "/api/users/99"
    );
    sleep(100);
    sheen::error!("user not found", id = 99);
    sleep(150);
    sheen::info!("response sent", status = 404, duration_ms = 12);
    sleep(500);

    sheen::info!("request received", method = "GET", path = "/health");
    sleep(50);
    sheen::trace!("health check passed", uptime_secs = 42);
    sleep(100);
    sheen::info!("response sent", status = 200, duration_ms = 2);
    sleep(400);

    // Graceful shutdown
    sheen::info!("shutdown signal received");
    sleep(200);
    sheen::debug!("closing database connections");
    sleep(300);
    sheen::debug!("flushing pending writes", count = 3);
    sleep(200);
    sheen::info!("server stopped gracefully");
}

fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
