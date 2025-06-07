pub mod applet;
pub mod config;
pub mod weather;

pub fn run() -> cosmic::iced::Result {
    applet::run()
}
