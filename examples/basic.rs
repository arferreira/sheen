fn main() {
    sheen::init_with(
        sheen::Logger::new()
            .level(sheen::Level::Trace)
            .prefix("myapp"),
    );

    sheen::trace!("now visible", id = 1);
    sheen::debug!("also visible", id = 2);
    sheen::info!("always visible", id = 3);
}
