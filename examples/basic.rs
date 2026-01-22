fn main() {
    sheen::init();
    sheen::info!("No fields");
    sheen::info!("With fields", port = 3000, host = "localhost");
}
