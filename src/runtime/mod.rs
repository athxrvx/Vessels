pub mod bollard_client;
pub mod discovery;

use crate::core::errors::VesselsError;
use crate::core::models::Container;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeInfo {
    pub name: String,
    pub version: String,
    pub endpoint: String,
    pub is_rootless: bool,
}

/// Common runtime interface as defined in PRD Section 11.4
#[async_trait]
pub trait ContainerRuntime: Send + Sync {
    async fn connect(&mut self) -> Result<RuntimeInfo, VesselsError>;
    async fn is_connected(&self) -> bool;
    async fn list_containers(&self) -> Result<Vec<Container>, VesselsError>;
    async fn inspect_container(&self, id: &str) -> Result<Container, VesselsError>;
    async fn start_container(&self, id: &str) -> Result<(), VesselsError>;
    async fn stop_container(&self, id: &str) -> Result<(), VesselsError>;
    async fn restart_container(&self, id: &str) -> Result<(), VesselsError>;
    async fn delete_container(&self, id: &str) -> Result<(), VesselsError>;
    fn runtime_name(&self) -> &str;
    fn endpoint(&self) -> &str;
}
