#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Containers,
    Overview,
    Hub,
    Projects,
    Editors,
    Settings,
}

impl Page {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Containers => "Containers",
            Self::Overview => "Overview",
            Self::Hub => "Hub",
            Self::Projects => "Projects",
            Self::Editors => "Editors",
            Self::Settings => "Settings",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Containers => "utilities-terminal-symbolic",
            Self::Overview => "view-dashboard-symbolic",
            Self::Hub => "system-software-install-symbolic",
            Self::Projects => "folder-development-symbolic",
            Self::Editors => "accessories-text-editor-symbolic",
            Self::Settings => "preferences-system-symbolic",
        }
    }
}
