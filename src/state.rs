//! The result of daemon detection.

/// Whether — and how — the current process runs as an OS daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonState {
    /// Whether the process is running as a daemon (parent is init /
    /// launchd / the Windows service manager, or the effective user
    /// is a configured service account).
    pub is_daemon: bool,
    /// The derived OS service label (e.g. a launchd label on macOS,
    /// a systemd template-unit name on Linux, a service name on
    /// Windows). `None` when the process is not a daemon.
    pub service_label: Option<String>,
    /// Whether the service runs system-wide or per-user. `None` when
    /// the process is not a daemon.
    pub service_mode: Option<DaemonServiceMode>,
}

impl DaemonState {
    /// The state of a process that is not running as a daemon.
    #[must_use]
    pub fn not_a_daemon() -> Self {
        Self {
            is_daemon: false,
            service_label: None,
            service_mode: None,
        }
    }
}

/// Whether a daemon service is installed system-wide or per-user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonServiceMode {
    /// System-scoped service (root or a dedicated service account).
    System,
    /// User-scoped service (the current user's launchd agent or
    /// `systemd --user` unit).
    User,
}
