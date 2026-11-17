use crate::app::Message;
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

pub fn view<'a>() -> Element<'a, Message> {
    let mut root = column().spacing(24).padding(24);

    // Header (PRD Section 7)
    root = root.push(
        row()
            .spacing(16)
            .align_y(cosmic::iced::Alignment::Center)
            .push(
                column()
                    .spacing(4)
                    .push(text::title1("Projects"))
                    .push(text("Manage development workspaces and container environments").size(14)),
            )
            .push(cosmic::widget::horizontal_space())
            .push(button::primary("New Project")),
    );

    // Empty state (PRD Section 14.6)
    root = root.push(
        container(
            column()
                .spacing(12)
                .align_x(cosmic::iced::Alignment::Center)
                .push(text::title3("No projects registered"))
                .push(
                    text("Import an existing directory with compose.yaml or create a new workspace.")
                        .size(14),
                )
                .push(
                    row()
                        .spacing(12)
                        .push(button::primary("Add Existing Project"))
                        .push(button::standard("Import Compose")),
                ),
        )
        .padding(48)
        .center_x(cosmic::iced::Length::Fill),
    );

    root.into()
}
