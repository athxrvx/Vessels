use crate::app::Message;
use cosmic::widget::{button, column, container, row, space, text};
use cosmic::Element;

pub fn view<'a>() -> Element<'a, Message> {
    let mut root = column::with_capacity(2).spacing(24).padding(24);

    // Header (PRD Section 7)
    root = root.push(
        row::with_capacity(3)
            .spacing(16)
            .align_y(cosmic::iced::Alignment::Center)
            .push(
                column::with_capacity(2)
                    .spacing(4)
                    .push(text::title1("Projects"))
                    .push(text("Manage development workspaces and container environments").size(14)),
            )
            .push(space::horizontal())
            .push(button::suggested("New Project")),
    );

    // Empty state (PRD Section 14.6)
    root = root.push(
        container(
            column::with_capacity(3)
                .spacing(12)
                .align_x(cosmic::iced::Alignment::Center)
                .push(text::title3("No projects registered"))
                .push(
                    text("Import an existing directory with compose.yaml or create a new workspace.")
                        .size(14),
                )
                .push(
                    row::with_capacity(2)
                        .spacing(12)
                        .push(button::suggested("Add Existing Project"))
                        .push(button::standard("Import Compose")),
                ),
        )
        .padding(48)
        .center_x(cosmic::iced::Length::Fill),
    );

    root.into()
}
