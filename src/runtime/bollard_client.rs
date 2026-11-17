use super::{ContainerRuntime, RuntimeInfo};
use crate::core::errors::VesselsError;
use crate::core::models::{Container, ContainerStatus, MountInfo, PortMapping};
use async_trait::async_trait;
use bollard::container::ListContainersOptions;
use bollard::Docker;
use std::collections::HashMap;

pub struct BollardRuntime {
    client: Option<Docker>,
    endpoint: String,
    runtime_name: String,
    info: Option<RuntimeInfo>,
}

impl BollardRuntime {
    pub fn new(endpoint: String, runtime_name: String) -> Self {
        Self {
            client: None,
            endpoint,
            runtime_name,
            info: None,
        }
    }

    pub fn auto_connect() -> Self {
        Self::new("auto".to_string(), "Podman/Docker".to_string())
    }
}

#[async_trait]
impl ContainerRuntime for BollardRuntime {
    async fn connect(&mut self) -> Result<RuntimeInfo, VesselsError> {
        let client = if self.endpoint == "auto" {
            Docker::connect_with_local_defaults().map_err(|e| VesselsError::RuntimeUnavailable {
                endpoint: "local-defaults".to_string(),
                details: e.to_string(),
            })?
        } else {
            Docker::connect_with_unix(&self.endpoint, 120, bollard::API_DEFAULT_VERSION).map_err(
                |e| VesselsError::RuntimeUnavailable {
                    endpoint: self.endpoint.clone(),
                    details: e.to_string(),
                },
            )?
        };

        let version_info = client
            .version()
            .await
            .map_err(|e| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: e.to_string(),
            })?;

        let version_str = version_info.version.unwrap_or_else(|| "unknown".to_string());
        let info = RuntimeInfo {
            name: self.runtime_name.clone(),
            version: version_str,
            endpoint: self.endpoint.clone(),
            is_rootless: true,
        };

        self.info = Some(info.clone());
        self.client = Some(client);
        Ok(info)
    }

    async fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    async fn list_containers(&self) -> Result<Vec<Container>, VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let raw_containers = client
            .list_containers(Some(options))
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "list".to_string(),
                id: "".to_string(),
                reason: e.to_string(),
            })?;

        let containers = raw_containers
            .into_iter()
            .map(|c| {
                let id = c.id.unwrap_or_default();
                let name = c
                    .names
                    .and_then(|names| names.into_iter().next())
                    .map(|n| n.trim_start_matches('/').to_string())
                    .unwrap_or_else(|| id[..12.min(id.len())].to_string());

                let image = c.image.unwrap_or_else(|| "unknown".to_string());

                let state_str = c.state.unwrap_or_default().to_lowercase();
                let status = match state_str.as_str() {
                    "running" => ContainerStatus::Running,
                    "exited" | "stopped" => ContainerStatus::Stopped,
                    "paused" => ContainerStatus::Paused,
                    "restarting" => ContainerStatus::Starting,
                    "dead" => ContainerStatus::Failed,
                    _ => ContainerStatus::Unknown,
                };

                let ports = c
                    .ports
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| PortMapping {
                        host_ip: p.ip,
                        host_port: p.public_port.unwrap_or(0),
                        container_port: p.private_port,
                        protocol: p.typ.map(|t| t.to_string()).unwrap_or_else(|| "tcp".to_string()),
                    })
                    .collect();

                let mounts = c
                    .mounts
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| MountInfo {
                        source: m.source.unwrap_or_default(),
                        destination: m.destination.unwrap_or_default(),
                        mode: m.mode.unwrap_or_default(),
                        is_rw: m.rw.unwrap_or(true),
                    })
                    .collect();

                Container {
                    id,
                    name,
                    image,
                    status,
                    created_at: c.created.unwrap_or(0),
                    ports,
                    mounts,
                    resource_usage: None,
                    group_id: None,
                    project_id: None,
                    environment_id: None,
                    service_id: None,
                }
            })
            .collect();

        Ok(containers)
    }

    async fn inspect_container(&self, id: &str) -> Result<Container, VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        let details = client
            .inspect_container(id, None)
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "inspect".to_string(),
                id: id.to_string(),
                reason: e.to_string(),
            })?;

        let name = details
            .name
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_else(|| id.to_string());

        let state_str = details
            .state
            .and_then(|s| s.status)
            .map(|st| st.to_string().to_lowercase())
            .unwrap_or_default();

        let status = match state_str.as_str() {
            "running" => ContainerStatus::Running,
            "exited" => ContainerStatus::Stopped,
            "paused" => ContainerStatus::Paused,
            "restarting" => ContainerStatus::Starting,
            "dead" => ContainerStatus::Failed,
            _ => ContainerStatus::Unknown,
        };

        Ok(Container {
            id: details.id.unwrap_or_else(|| id.to_string()),
            name,
            image: details
                .config
                .and_then(|c| c.image)
                .unwrap_or_else(|| "unknown".to_string()),
            status,
            created_at: 0,
            ports: Vec::new(),
            mounts: Vec::new(),
            resource_usage: None,
            group_id: None,
            project_id: None,
            environment_id: None,
            service_id: None,
        })
    }

    async fn start_container(&self, id: &str) -> Result<(), VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        client
            .start_container(id, None::<bollard::container::StartContainerOptions<String>>)
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "start".to_string(),
                id: id.to_string(),
                reason: e.to_string(),
            })
    }

    async fn stop_container(&self, id: &str) -> Result<(), VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        client
            .stop_container(id, None)
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "stop".to_string(),
                id: id.to_string(),
                reason: e.to_string(),
            })
    }

    async fn restart_container(&self, id: &str) -> Result<(), VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        client
            .restart_container(id, None)
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "restart".to_string(),
                id: id.to_string(),
                reason: e.to_string(),
            })
    }

    async fn delete_container(&self, id: &str) -> Result<(), VesselsError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| VesselsError::RuntimeUnavailable {
                endpoint: self.endpoint.clone(),
                details: "Not connected to runtime".to_string(),
            })?;

        client
            .remove_container(id, None)
            .await
            .map_err(|e| VesselsError::ContainerOperationFailed {
                op: "remove".to_string(),
                id: id.to_string(),
                reason: e.to_string(),
            })
    }

    fn runtime_name(&self) -> &str {
        &self.runtime_name
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
