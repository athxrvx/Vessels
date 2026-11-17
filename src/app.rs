use crate::core::config::AppConfig;
use crate::core::models::{Container, ContainerStatus};
use crate::runtime::bollard_client::BollardRuntime;
use crate::runtime::{ContainerRuntime, RuntimeInfo};
use crate::ui::navigation::Page;
use crate::ui::pages;

use cosmic::app::Core;
use cosmic::executor;
use cosmic::widget::icon;
use cosmic::widget::nav_bar;
use cosmic::{Action, Application, ApplicationExt, Element, Task};

pub struct AppModel {
    core: Core,
    nav_model: nav_bar::Model,
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
    #[allow(dead_code)]
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

        // Native COSMIC nav_bar::Model matching system design guidelines
        let nav_model = nav_bar::Model::builder()
            .insert(|b| {
                b.text(Page::Containers.title())
                    .icon(icon::from_name("utilities-terminal-symbolic"))
                    .data(Page::Containers)
                    .activate()
            })
            .insert(|b| {
                b.text(Page::Overview.title())
                    .icon(icon::from_name("view-dashboard-symbolic"))
                    .data(Page::Overview)
            })
            .insert(|b| {
                b.text(Page::Hub.title())
                    .icon(icon::from_name("system-software-install-symbolic"))
                    .data(Page::Hub)
            })
            .insert(|b| {
                b.text(Page::Projects.title())
                    .icon(icon::from_name("folder-development-symbolic"))
                    .data(Page::Projects)
            })
            .insert(|b| {
                b.text(Page::Editors.title())
                    .icon(icon::from_name("accessories-text-editor-symbolic"))
                    .data(Page::Editors)
                    .divider_above()
            })
            .insert(|b| {
                b.text(Page::Settings.title())
                    .icon(icon::from_name("preferences-system-symbolic"))
                    .data(Page::Settings)
            })
            .build();

        let mut app = Self {
            core,
            nav_model,
            active_page: Page::Containers, // PRD Section 3.1: Default landing page is Containers
            config,
            containers: Vec::new(),
            runtime_info: None,
            search_query: String::new(),
            status_filter: None,
            is_loading: true,
            error_message: None,
        };

        app.set_header_title("Containers — Vessels".into());

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

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Action<Self::Message>> {
        self.nav_model.activate(id);
        if let Some(page) = self.nav_model.data::<Page>(id) {
            self.active_page = *page;
            self.set_header_title(format!("{} — Vessels", page.title()));
        }
        Task::none()
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Navigate(page) => {
                self.active_page = page;
                self.set_header_title(format!("{} — Vessels", page.title()));
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
                        self.error_message = None;
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
        // Render the active view. COSMIC automatically attaches the native nav_bar in the chrome.
        match self.active_page {
            Page::Containers => pages::containers::view(
                &self.containers,
                &self.search_query,
                self.status_filter,
                self.is_loading,
                self.runtime_info.as_ref(),
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
        }
    }
}
