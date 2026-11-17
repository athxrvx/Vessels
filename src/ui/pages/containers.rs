use crate::app::Message;
use crate::core::models::{Container, ContainerStatus};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input};
use cosmic::Element;

pub fn view<'a>(
    containers: &'a [Container],
    search_query: &'a str,
    selected_status_filter: Option<ContainerStatus>,
    is_loading: bool,
) -> Element<'a, Message> {
    let mut root = column().spacing(24).padding(24);

    // 1. Header Area (PRD Section 4.1)
    let header = row()
        .spacing(16)
        .align_y(cosmic::iced::Alignment::Center)
        .push(
            column()
                .spacing(4)
                .push(text::title1("Containers"))
                .push(text("Manage individual containers and manual groups").size(14)),
        )
        .push(cosmic::widget::horizontal_space())
        .push(
            button::standard("Refresh")
                .on_press(Message::RefreshContainers),
        );
    root = root.push(header);

    // 2. Summary Cards (Total, Running, Stopped - PRD Section 4.1)
    let total_count = containers.len();
    let running_count = containers
        .iter()
        .filter(|c| c.status == ContainerStatus::Running)
        .count();
    let stopped_count = containers
        .iter()
        .filter(|c| c.status == ContainerStatus::Stopped)
        .count();

    let summary_row = row()
        .spacing(16)
        .push(summary_card("Total", total_count, None, selected_status_filter == None))
        .push(summary_card(
            "Running",
            running_count,
            Some(ContainerStatus::Running),
            selected_status_filter == Some(ContainerStatus::Running),
        ))
        .push(summary_card(
            "Stopped",
            stopped_count,
            Some(ContainerStatus::Stopped),
            selected_status_filter == Some(ContainerStatus::Stopped),
        ));
    root = root.push(summary_row);

    // 3. Search and Filter Bar
    let search_bar = row()
        .spacing(12)
        .align_y(cosmic::iced::Alignment::Center)
        .push(
            text_input("Search containers or images...", search_query)
                .on_input(Message::SearchQueryChanged)
                .width(cosmic::iced::Length::FillPortion(2)),
        );
    root = root.push(search_bar);

    // 4. Containers List / Empty State
    let filtered_containers: Vec<&Container> = containers
        .iter()
        .filter(|c| {
            if let Some(status) = selected_status_filter {
                if c.status != status {
                    return false;
                }
            }
            if !search_query.is_empty() {
                let q = search_query.to_lowercase();
                let match_name = c.name.to_lowercase().contains(&q);
                let match_image = c.image.to_lowercase().contains(&q);
                return match_name || match_image;
            }
            true
        })
        .collect();

    if filtered_containers.is_empty() {
        if is_loading {
            root = root.push(
                container(
                    column()
                        .spacing(12)
                        .align_x(cosmic::iced::Alignment::Center)
                        .push(text("Connecting to runtime and loading containers...").size(16)),
                )
                .padding(40)
                .center_x(cosmic::iced::Length::Fill)
                .center_y(cosmic::iced::Length::Fill),
            );
        } else {
            // Empty State (PRD Section 14.6)
            root = root.push(
                container(
                    column()
                        .spacing(12)
                        .align_x(cosmic::iced::Alignment::Center)
                        .push(text::title3("No containers found"))
                        .push(
                            text("Containers will appear here once created or pulled from registries.")
                                .size(14),
                        )
                        .push(
                            row()
                                .spacing(12)
                                .push(
                                    button::primary("Explore Hub")
                                        .on_press(Message::Navigate(crate::ui::navigation::Page::Hub)),
                                )
                                .push(
                                    button::standard("Refresh")
                                        .on_press(Message::RefreshContainers),
                                ),
                        ),
                )
                .padding(48)
                .center_x(cosmic::iced::Length::Fill),
            );
        }
    } else {
        let mut list_column = column().spacing(12);
        for container_item in filtered_containers {
            list_column = list_column.push(container_card(container_item));
        }
        root = root.push(scrollable(list_column));
    }

    root.into()
}

fn summary_card<'a>(
    label: &'static str,
    count: usize,
    filter: Option<ContainerStatus>,
    is_active: bool,
) -> Element<'a, Message> {
    let content = column()
        .spacing(4)
        .push(text(label).size(12))
        .push(text::title2(count.to_string()));

    let mut btn = button::standard(content)
        .on_press(Message::FilterStatus(filter))
        .width(cosmic::iced::Length::Fill);

    if is_active {
        btn = btn.class(cosmic::theme::Button::Suggested);
    }

    btn.into()
}

fn container_card<'a>(c: &'a Container) -> Element<'a, Message> {
    let status_badge = text(format!("● {}", c.status)).size(13);

    let info_column = column()
        .spacing(4)
        .push(
            row()
                .spacing(10)
                .align_y(cosmic::iced::Alignment::Center)
                .push(text::title3(&c.name))
                .push(status_badge),
        )
        .push(
            text(format!("Image: {}", c.image))
                .size(13),
        );

    // Lifecycle Action Buttons (PRD Section 4.1)
    let id_clone_start = c.id.clone();
    let id_clone_stop = c.id.clone();
    let id_clone_restart = c.id.clone();
    let id_clone_delete = c.id.clone();

    let mut actions = row().spacing(8).align_y(cosmic::iced::Alignment::Center);

    if c.status == ContainerStatus::Running {
        actions = actions
            .push(
                button::standard("Stop")
                    .on_press(Message::StopContainer(id_clone_stop)),
            )
            .push(
                button::standard("Restart")
                    .on_press(Message::RestartContainer(id_clone_restart)),
            );
    } else {
        actions = actions
            .push(
                button::primary("Start")
                    .on_press(Message::StartContainer(id_clone_start)),
            );
    }

    actions = actions.push(
        button::destructive("Delete")
            .on_press(Message::DeleteContainer(id_clone_delete)),
    );

    let card_layout = row()
        .spacing(16)
        .padding(16)
        .align_y(cosmic::iced::Alignment::Center)
        .push(info_column)
        .push(cosmic::widget::horizontal_space())
        .push(actions);

    container(card_layout)
        .class(cosmic::theme::Container::Card)
        .width(cosmic::iced::Length::Fill)
        .into()
}
