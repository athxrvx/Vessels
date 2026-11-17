use crate::core::config::AppConfig;
use crate::core::models::{Container, ContainerStatus};
use crate::runtime::bollard_client::BollardRuntime;
use crate::runtime::{ContainerRuntime, RuntimeInfo};
use crate::ui::navigation::Page;
use crate::ui::pages;

use cosmic::app::Core;
use cosmic::executor;
use cosmic::widget::{button, column, container, divider, row, space, text};
use cosmic::{Action, Application, Element, Task};

pub struct AppModel {
    core: Core,
    active_page: Page,
    config: AppConfig,
    containers: Vec<Container>,
    runtime_info: Option<RuntimeInfo>,
    search_query: String,
    status_filter: Option<ContainerStatus>,
    is_loading: bool,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    ConnectRuntime,
    RuntimeConnected(Result<RuntimeInfo, String>),
    RefreshContainers,
    ContainersLoaded(Result<Vec<Container>, String>),
    StartContainer(String),
    StopContainer(String),
    RestartContainer(String),
    DeleteContainer(String),
    ContainerActionCompleted(Result<String, String>),
    SearchQueryChanged(String),
    FilterStatus(Option<ContainerStatus>),
    DismissError,
}

impl Application for AppModel {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.atharva.Vessels";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let config = AppConfig::load();
        let app = Self {
            core,
            active_page: Page::Containers, // PRD Section 3.1: Default landing page is Containers
            config,
            containers: Vec::new(),
            runtime_info: None,
            search_query: String::new(),
            status_filter: None,
            is_loading: true,
            error_message: None,
        };

        // Automatically connect to container runtime on startup
        let task = Task::perform(
            async {
                let mut runtime = BollardRuntime::auto_connect();
                match runtime.connect().await {
                    Ok(info) => Ok(info),
                    Err(e) => Err(e.to_string()),
                }
            },
            |res| Action::from(Message::RuntimeConnected(res)),
        );

        (app, task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Navigate(page) => {
                self.active_page = page;
                Task::none()
            }
            Message::ConnectRuntime => {
                self.is_loading = true;
                self.error_message = None;
                Task::perform(
                    async {
                        let mut runtime = BollardRuntime::auto_connect();
                        match runtime.connect().await {
                            Ok(info) => Ok(info),
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    |res| Action::from(Message::RuntimeConnected(res)),
                )
            }
            Message::RuntimeConnected(res) => {
                self.is_loading = false;
                match res {
                    Ok(info) => {
                        self.runtime_info = Some(info);
                        // Trigger immediate container refresh upon successful connection
                        self.update(Message::RefreshContainers)
                    }
                    Err(e) => {
                        self.runtime_info = None;
                        self.error_message = Some(e);
                        Task::none()
                    }
                }
            }
            Message::RefreshContainers => {
                self.is_loading = true;
                Task::perform(
                    async {
                        let mut runtime = BollardRuntime::auto_connect();
                        if let Err(e) = runtime.connect().await {
                            return Err(e.to_string());
                        }
                        runtime.list_containers().await.map_err(|e| e.to_string())
                    },
                    |res| Action::from(Message::ContainersLoaded(res)),
                )
            }
            Message::ContainersLoaded(res) => {
                self.is_loading = false;
                match res {
                    Ok(containers) => {
                        self.containers = containers;
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                    }
                }
                Task::none()
            }
            Message::StartContainer(id) => {
                self.is_loading = true;
                Task::perform(
                    async move {
                        let mut runtime = BollardRuntime::auto_connect();
                        runtime.connect().await.map_err(|e| e.to_string())?;
                        runtime
                            .start_container(&id)
                            .await
                            .map(|_| format!("Started container {}", id))
                            .map_err(|e| e.to_string())
                    },
                    |res| Action::from(Message::ContainerActionCompleted(res)),
                )
            }
            Message::StopContainer(id) => {
                self.is_loading = true;
                Task::perform(
                    async move {
                        let mut runtime = BollardRuntime::auto_connect();
                        runtime.connect().await.map_err(|e| e.to_string())?;
                        runtime
                            .stop_container(&id)
                            .await
                            .map(|_| format!("Stopped container {}", id))
                            .map_err(|e| e.to_string())
                    },
                    |res| Action::from(Message::ContainerActionCompleted(res)),
                )
            }
            Message::RestartContainer(id) => {
                self.is_loading = true;
                Task::perform(
                    async move {
                        let mut runtime = BollardRuntime::auto_connect();
                        runtime.connect().await.map_err(|e| e.to_string())?;
                        runtime
                            .restart_container(&id)
                            .await
                            .map(|_| format!("Restarted container {}", id))
                            .map_err(|e| e.to_string())
                    },
                    |res| Action::from(Message::ContainerActionCompleted(res)),
                )
            }
            Message::DeleteContainer(id) => {
                self.is_loading = true;
                Task::perform(
                    async move {
                        let mut runtime = BollardRuntime::auto_connect();
                        runtime.connect().await.map_err(|e| e.to_string())?;
                        runtime
                            .delete_container(&id)
                            .await
                            .map(|_| format!("Deleted container {}", id))
                            .map_err(|e| e.to_string())
                    },
                    |res| Action::from(Message::ContainerActionCompleted(res)),
                )
            }
            Message::ContainerActionCompleted(res) => {
                self.is_loading = false;
                match res {
                    Ok(_) => self.update(Message::RefreshContainers),
                    Err(e) => {
                        self.error_message = Some(e);
                        Task::none()
                    }
                }
            }
            Message::SearchQueryChanged(q) => {
                self.search_query = q;
                Task::none()
            }
            Message::FilterStatus(filter) => {
                self.status_filter = filter;
                Task::none()
            }
            Message::DismissError => {
                self.error_message = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Navigation Sidebar (PRD Section 3.2)
        let sidebar = column::with_capacity(8)
            .spacing(8)
            .padding(12)
            .width(cosmic::iced::Length::Fixed(200.0))
            // Primary destinations
            .push(self.nav_button(Page::Containers))
            .push(self.nav_button(Page::Overview))
            .push(self.nav_button(Page::Hub))
            .push(self.nav_button(Page::Projects))
            // Subtle divider (PRD Section 3.2)
            .push(divider::horizontal::default())
            // Secondary destinations
            .push(self.nav_button(Page::Editors))
            .push(self.nav_button(Page::Settings));

        // Active page body
        let page_content: Element<'_, Message> = match self.active_page {
            Page::Containers => pages::containers::view(
                &self.containers,
                &self.search_query,
                self.status_filter,
                self.is_loading,
            ),
            Page::Overview => pages::overview::view(
                self.runtime_info.as_ref(),
                self.containers.len(),
                self.containers
                    .iter()
                    .filter(|c| c.status == ContainerStatus::Running)
                    .count(),
            ),
            Page::Hub => pages::hub::view(&self.search_query),
            Page::Projects => pages::projects::view(),
            Page::Editors => pages::editors::view(),
            Page::Settings => pages::settings::view(&self.config),
        };

        // Main layout with optional error banner
        let mut main_content = column::with_capacity(2).width(cosmic::iced::Length::Fill);

        if let Some(ref err) = self.error_message {
            let error_banner = container(
                row::with_capacity(3)
                    .spacing(12)
                    .align_y(cosmic::iced::Alignment::Center)
                    .push(text(format!("Error: {}", err)).size(13))
                    .push(space::horizontal())
                    .push(button::standard("Dismiss").on_press(Message::DismissError)),
            )
            .class(cosmic::theme::Container::Card)
            .padding(12)
            .width(cosmic::iced::Length::Fill);

            main_content = main_content.push(error_banner);
        }

        main_content = main_content.push(page_content);

        let app_layout = row::with_capacity(2)
            .push(sidebar)
            .push(main_content);

        container(app_layout)
            .width(cosmic::iced::Length::Fill)
            .height(cosmic::iced::Length::Fill)
            .into()
    }
}

impl AppModel {
    fn nav_button<'a>(&self, page: Page) -> Element<'a, Message> {
        let is_active = self.active_page == page;
        let mut btn = button::standard(page.title())
            .on_press(Message::Navigate(page))
            .width(cosmic::iced::Length::Fill);

        if is_active {
            btn = btn.class(cosmic::theme::Button::Suggested);
        }

        btn.into()
    }
}
