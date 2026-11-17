use crate::app::Message;
use cosmic::widget::{column, container, row, text, text_input};
use cosmic::Element;

pub fn view<'a>(search_query: &'a str) -> Element<'a, Message> {
    let mut root = column::with_capacity(4).spacing(24).padding(24);

    // Header (PRD Section 6)
    root = root.push(
        column::with_capacity(2)
            .spacing(4)
            .push(text::title1("Hub"))
            .push(text("Discover, inspect and configure OCI container images").size(14)),
    );

    // Search bar
    root = root.push(
        text_input("Search Docker Hub, Quay.io, GHCR...", search_query)
            .width(cosmic::iced::Length::Fill),
    );

    // Categories (PRD Section 6.2)
    let categories = row::with_capacity(4)
        .spacing(12)
        .push(category_pill("Linux Distributions"))
        .push(category_pill("Programming Languages"))
        .push(category_pill("Databases"))
        .push(category_pill("Web Servers"));

    root = root.push(categories);

    // Placeholder message
    root = root.push(
        container(
            column::with_capacity(2)
                .spacing(10)
                .align_x(cosmic::iced::Alignment::Center)
                .push(text::title3("Registry Discovery"))
                .push(text("Connect to registries or pull standard development images.").size(14)),
        )
        .padding(40)
        .center_x(cosmic::iced::Length::Fill),
    );

    root.into()
}

fn category_pill<'a>(name: &'static str) -> Element<'a, Message> {
    container(text(name).size(13))
        .class(cosmic::theme::Container::Card)
        .padding(10)
        .into()
}
