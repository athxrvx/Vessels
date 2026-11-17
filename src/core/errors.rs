use thiserror::Error;

/// Structured, actionable domain errors across all Vessels subsystems (PRD Section 16)
#[derive(Debug, Error)]
pub enum VesselsError {
    #[error("Runtime is unavailable at '{endpoint}': {details}. Please check that the daemon is running.")]
    RuntimeUnavailable {
        endpoint: String,
        details: String,
    },

    #[error("Failed to connect to runtime socket '{path}': {source}")]
    SocketConnectionFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to pull image '{image}': {reason}")]
    ImagePullFailed {
        image: String,
        reason: String,
    },

    #[error("Container '{name}' failed to start: {reason}")]
    ContainerStartupFailed {
        name: String,
        reason: String,
    },

    #[error("Container operation '{op}' failed on container '{id}': {reason}")]
    ContainerOperationFailed {
        op: String,
        id: String,
        reason: String,
    },

    #[error("Flatpak sandbox permission denied: missing access to '{resource}'. {resolution}")]
    PermissionDenied {
        resource: String,
        resolution: String,
    },

    #[error("Editor '{name}' is unavailable: {reason}")]
    EditorUnavailable {
        name: String,
        reason: String,
    },

    #[error("Configuration error in '{path}': {reason}")]
    ConfigError {
        path: String,
        reason: String,
    },

    #[error("Project '{name}' error: {reason}")]
    ProjectError {
        name: String,
        reason: String,
    },

    #[error("Unknown error: {0}")]
    Other(String),
}
