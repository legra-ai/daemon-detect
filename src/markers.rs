//! The caller-supplied indicators that identify a daemon environment.

/// Environment markers that count as evidence the process is a daemon.
///
/// Two kinds of evidence are configurable on top of the built-in
/// parent-process heuristics (parent PID 1 on Unix; on Windows, the
/// built-ins alone never fire — supply an environment marker):
///
/// - **Service accounts** — user account names whose effective UID marks the
///   process as a system daemon on Unix (e.g. `_mydaemon` on macOS, `mydaemon`
///   on Linux).
/// - **A Windows service environment variable** — a variable the service
///   wrapper injects into the daemon's environment.
#[derive(Debug, Clone, Default)]
pub struct DaemonMarkers {
    pub(crate) service_accounts: Vec<String>,
    pub(crate) windows_service_env: Option<String>,
}

impl DaemonMarkers {
    /// Markers with no service accounts and no Windows environment
    /// variable: only the parent-process heuristics apply.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a Unix service-account name whose effective UID marks the
    /// process as a system daemon.
    #[must_use]
    pub fn service_account(mut self, name: impl Into<String>) -> Self {
        self.service_accounts.push(name.into());
        self
    }

    /// Set the environment variable that a Windows service wrapper
    /// injects; its presence marks the process as a service.
    #[must_use]
    pub fn windows_service_env(mut self, variable: impl Into<String>) -> Self {
        self.windows_service_env = Some(variable.into());
        self
    }

    /// Whether the current process is likely running as a daemon.
    ///
    /// Suitable for early startup decisions (e.g. telemetry routing)
    /// before a full service identity exists. Does not derive labels.
    #[must_use]
    pub fn is_daemon(&self) -> bool {
        crate::platform::is_daemon_impl(self)
    }
}
