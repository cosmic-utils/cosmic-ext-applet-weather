fn main() -> cosmic::iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting weather applet");

    cosmic_ext_applet_weather::run()
}
