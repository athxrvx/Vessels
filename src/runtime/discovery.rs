#![allow(dead_code)]

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveredSocket {
    RootlessPodman(PathBuf),
    SystemPodman(PathBuf),
    Docker(PathBuf),
}

impl DiscoveredSocket {
    pub fn path(&self) -> &Path {
        match self {
            Self::RootlessPodman(p) | Self::SystemPodman(p) | Self::Docker(p) => p,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::RootlessPodman(_) => "Podman (Rootless)",
            Self::SystemPodman(_) => "Podman (System)",
            Self::Docker(_) => "Docker",
        }
    }
}

/// Auto-discovers container engine sockets according to PRD section 2.2 and 13.2
pub fn discover_runtime_sockets() -> Vec<DiscoveredSocket> {
    let mut discovered = Vec::new();

    // 1. Rootless Podman via XDG_RUNTIME_DIR
    if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let xdg_podman = PathBuf::from(&xdg_runtime).join("podman/podman.sock");
        if xdg_podman.exists() {
            discovered.push(DiscoveredSocket::RootlessPodman(xdg_podman));
        }

        let xdg_docker = PathBuf::from(&xdg_runtime).join("docker.sock");
        if xdg_docker.exists() {
            discovered.push(DiscoveredSocket::Docker(xdg_docker));
        }
    }

    // 2. Fallback UID-based rootless Podman
    let uid = libc_uid();
    let uid_podman = PathBuf::from(format!("/run/user/{uid}/podman/podman.sock"));
    if uid_podman.exists() && !discovered.iter().any(|d| d.path() == uid_podman.as_path()) {
        discovered.push(DiscoveredSocket::RootlessPodman(uid_podman));
    }

    // 3. System-wide Podman socket
    let system_podman = PathBuf::from("/run/podman/podman.sock");
    if system_podman.exists() {
        discovered.push(DiscoveredSocket::SystemPodman(system_podman));
    }

    // 4. Standard Docker socket
    let system_docker = PathBuf::from("/var/run/docker.sock");
    if system_docker.exists() {
        discovered.push(DiscoveredSocket::Docker(system_docker));
    }

    discovered
}

fn libc_uid() -> u32 {
    // Standard Unix getuid fallback
    std::env::var("UID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}
