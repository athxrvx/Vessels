mod app;
mod core;
mod runtime;
mod ui;

use app::AppModel;
use cosmic::app::Settings;
use cosmic::iced::Size;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging (PRD Section 11.2)
    tracing_subscriber::fmt::init();

    // Configure window settings with default size
    let settings = Settings::default().size(Size::new(1024., 720.));

    // Launch COSMIC application loop
    cosmic::app::run::<AppModel>(settings, ())?;

    Ok(())
}
