use crate::app::Message;
use crate::runtime::RuntimeInfo;
use cosmic::widget::{button, column, container, row, space, text};
use cosmic::Element;

pub fn view<'a>(
    runtime_info: Option<&'a RuntimeInfo>,
    container_count: usize,
    running_count: usize,
) -> Element<'a, Message> {
    let mut root = column::with_capacity(4).spacing(24).padding(24);

    // Header
    root = root.push(
        column::with_capacity(2)
            .spacing(4)
            .push(text::title1("Overview"))
            .push(text("Central operational command center").size(14)),
    );

    // Runtime Health Card (PRD Section 5.2)
    let runtime_status = match runtime_info {
        Some(info) => {
            row::with_capacity(3)
                .spacing(16)
                .align_y(cosmic::iced::Alignment::Center)
                .push(
                    column::with_capacity(2)
                        .spacing(4)
                        .push(text(format!("Runtime: {}", info.name)).size(16))
                        .push(text(format!("Version: {} | Endpoint: {}", info.version, info.endpoint)).size(13)),
                )
                .push(space::horizontal())
                .push(
                    button::standard("Reconnect")
                        .on_press(Message::ConnectRuntime),
                )
        }
        None => {
            row::with_capacity(3)
                .spacing(16)
                .align_y(cosmic::iced::Alignment::Center)
                .push(
                    column::with_capacity(2)
                        .spacing(4)
                        .push(text("Runtime: Disconnected").size(16))
                        .push(text("No active Podman or Docker socket connection detected").size(13)),
                )
                .push(space::horizontal())
                .push(
                    button::suggested("Connect")
                        .on_press(Message::ConnectRuntime),
                )
        }
    };

    let runtime_card = container(runtime_status)
        .class(cosmic::theme::Container::Card)
        .padding(20)
        .width(cosmic::iced::Length::Fill);

    root = root.push(runtime_card);

    // Quick Stats (PRD Section 5.3)
    let stats_row = row::with_capacity(3)
        .spacing(16)
        .push(stat_box("Total Containers", container_count.to_string()))
        .push(stat_box("Active Containers", running_count.to_string()))
        .push(stat_box("Active Workspaces", "0".to_string()));

    root = root.push(stats_row);

    // Quick Actions (PRD Section 5.8)
    let quick_actions = column::with_capacity(2)
        .spacing(12)
        .push(text::title3("Quick Actions"))
        .push(
            row::with_capacity(3)
                .spacing(12)
                .push(
                    button::standard("Explore Images (Hub)")
                        .on_press(Message::Navigate(crate::ui::navigation::Page::Hub)),
                )
                .push(
                    button::standard("Open Projects")
                        .on_press(Message::Navigate(crate::ui::navigation::Page::Projects)),
                )
                .push(
                    button::standard("Configure Settings")
                        .on_press(Message::Navigate(crate::ui::navigation::Page::Settings)),
                ),
        );

    root = root.push(quick_actions);

    root.into()
}

fn stat_box<'a>(label: &'static str, value: String) -> Element<'a, Message> {
    container(
        column::with_capacity(2)
            .spacing(6)
            .push(text(label).size(13))
            .push(text::title2(value)),
    )
    .class(cosmic::theme::Container::Card)
    .padding(16)
    .width(cosmic::iced::Length::Fill)
    .into()
}
