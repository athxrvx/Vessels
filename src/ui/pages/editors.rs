use crate::app::Message;
use cosmic::widget::{column, container, row, space, text};
use cosmic::Element;

pub fn view<'a>() -> Element<'a, Message> {
    let mut root = column::with_capacity(2).spacing(24).padding(24);

    // Header (PRD Section 8)
    root = root.push(
        column::with_capacity(2)
            .spacing(4)
            .push(text::title1("Editors"))
            .push(text("Detect installed host editors and configure remote development").size(14)),
    );

    // Detected Editors list
    let editors_col = column::with_capacity(4)
        .spacing(12)
        .push(editor_row("Visual Studio Code", "code", true, true))
        .push(editor_row("Zed", "dev.zed.Zed", true, false))
        .push(editor_row("COSMIC Terminal", "cosmic-term", true, true))
        .push(editor_row("Neovim", "nvim", true, true));

    root = root.push(editors_col);

    root.into()
}

fn editor_row<'a>(
    name: &'static str,
    id: &'static str,
    detected: bool,
    remote_capable: bool,
) -> Element<'a, Message> {
    let status_str = if detected {
        if remote_capable {
            "Detected (Remote Capable)"
        } else {
            "Detected (Local Launch)"
        }
    } else {
        "Not Detected"
    };

    container(
        row::with_capacity(3)
            .spacing(16)
            .padding(16)
            .align_y(cosmic::iced::Alignment::Center)
            .push(
                column::with_capacity(2)
                    .spacing(4)
                    .push(text::title3(name))
                    .push(text(format!("App ID: {}", id)).size(13)),
            )
            .push(space::horizontal())
            .push(text(status_str).size(13)),
    )
    .class(cosmic::theme::Container::Card)
    .width(cosmic::iced::Length::Fill)
    .into()
}
