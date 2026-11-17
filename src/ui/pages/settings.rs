use crate::app::Message;
use crate::core::config::AppConfig;
use cosmic::widget::{column, container, text};
use cosmic::Element;

pub fn view<'a>(config: &'a AppConfig) -> Element<'a, Message> {
    let mut root = column::with_capacity(3).spacing(24).padding(24);

    // Header (PRD Section 9)
    root = root.push(
        column::with_capacity(2)
            .spacing(4)
            .push(text::title1("Settings"))
            .push(text("Application and container runtime preferences").size(14)),
    );

    // Runtime Settings Card
    let runtime_card = container(
        column::with_capacity(3)
            .spacing(12)
            .push(text::title3("Runtime Configuration"))
            .push(text("Target: Auto-discovery (Podman rootless first, Docker fallback)").size(14))
            .push(text(format!("Refresh Interval: {} seconds", config.refresh_interval_secs)).size(13)),
    )
    .class(cosmic::theme::Container::Card)
    .padding(20)
    .width(cosmic::iced::Length::Fill);

    root = root.push(runtime_card);

    // Appearance Settings Card
    let appearance_card = container(
        column::with_capacity(3)
            .spacing(12)
            .push(text::title3("Appearance"))
            .push(text("Theme: Integrated with COSMIC Desktop system settings").size(14))
            .push(text("Density: Comfortable card layout").size(13)),
    )
    .class(cosmic::theme::Container::Card)
    .padding(20)
    .width(cosmic::iced::Length::Fill);

    root = root.push(appearance_card);

    root.into()
}
