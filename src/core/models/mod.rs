#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Container runtime status according to PRD section 14.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Paused,
    Unhealthy,
    Failed,
    Unknown,
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "Running"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Starting => write!(f, "Starting"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Paused => write!(f, "Paused"),
            Self::Unhealthy => write!(f, "Unhealthy"),
            Self::Failed => write!(f, "Failed"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Port mapping between container and host
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortMapping {
    pub host_ip: Option<String>,
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String,
}

/// Storage mount information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MountInfo {
    pub source: String,
    pub destination: String,
    pub mode: String,
    pub is_rw: bool,
}

/// Real-time resource metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_percentage: f32,
    pub memory_used_bytes: u64,
    pub memory_limit_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
}

/// Section 12.2: Represents an individual container instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub created_at: i64,
    pub ports: Vec<PortMapping>,
    pub mounts: Vec<MountInfo>,
    pub resource_usage: Option<ResourceUsage>,
    pub group_id: Option<String>,
    pub project_id: Option<String>,
    pub environment_id: Option<String>,
    pub service_id: Option<String>,
}

/// Section 12.3: Manual organizational container group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub container_ids: Vec<String>,
    pub display_order: u32,
    pub created_at: i64,
}

/// Section 12.1: Represents an OCI image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub registry: String,
    pub repository: String,
    pub tag: String,
    pub digest: Option<String>,
    pub architecture: Option<String>,
    pub os: Option<String>,
    pub size_bytes: u64,
    pub is_local: bool,
}

/// Section 12.4: Development workspace project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub root_dir: String,
    pub default_editor_id: Option<String>,
    pub default_environment_id: Option<String>,
    pub config_path: String,
    pub created_at: i64,
    pub last_opened_at: i64,
}

/// Section 12.5: Orchestrated environment inside a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub compose_file: Option<String>,
    pub service_ids: Vec<String>,
    pub status: String,
}

/// Section 12.6: Workload within an environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub environment_id: String,
    pub name: String,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub ports: Vec<PortMapping>,
    pub container_id: Option<String>,
    pub is_running: bool,
}

/// Section 12.7: Discovered host editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editor {
    pub id: String,
    pub name: String,
    pub app_id: String,
    pub exec: String,
    pub version: Option<String>,
    pub is_detected: bool,
    pub is_launchable: bool,
    pub remote_capable: bool,
}

/// Section 12.8: OCI Registry integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub is_authenticated: bool,
}
