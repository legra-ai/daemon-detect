//! The service identity a daemon detects itself under.

use crate::markers::DaemonMarkers;
use crate::state::DaemonState;
use crate::values::{
    BinaryName,
    LaunchdLabelPrefix,
    ServiceInstance,
};

/// How this process is named as an OS service.
///
/// Combines the binary name (used by systemd and Windows), a
/// reverse-DNS label prefix (used by launchd), and an instance
/// discriminator (a port, an identity key, …) so one binary can run
/// multiple service instances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceIdentity {
    binary_name: BinaryName,
    launchd_prefix: LaunchdLabelPrefix,
    instance: ServiceInstance,
}

impl ServiceIdentity {
    /// Describe a service instance from already-validated values.
    ///
    /// - `binary_name` — the installed binary, e.g. `my-daemon`;
    /// - `launchd_prefix` — the reverse-DNS launchd prefix, e.g.
    ///   `com.example.my-daemon`;
    /// - `instance` — the per-instance suffix, e.g. an HTTP port or a node
    ///   identity key. A `u16` port converts directly.
    #[must_use]
    pub fn new(
        binary_name: BinaryName,
        launchd_prefix: LaunchdLabelPrefix,
        instance: impl Into<ServiceInstance>,
    ) -> Self {
        Self {
            binary_name,
            launchd_prefix,
            instance: instance.into(),
        }
    }

    /// The macOS launchd label: `{launchd_prefix}.{instance}`.
    #[must_use]
    pub fn launchd_label(&self) -> String {
        format!("{}.{}", self.launchd_prefix, self.instance)
    }

    /// The Linux systemd template-unit name:
    /// `{binary_name}@{instance}.service`.
    #[must_use]
    pub fn systemd_unit_name(&self) -> String {
        format!("{}@{}.service", self.binary_name, self.instance)
    }

    /// The Windows service name: `{binary_name}-{instance}`.
    #[must_use]
    pub fn windows_service_name(&self) -> String {
        format!("{}-{}", self.binary_name, self.instance)
    }

    /// The service label for the platform this process runs on.
    #[must_use]
    pub fn service_label(&self) -> String {
        if cfg!(target_os = "macos") {
            self.launchd_label()
        } else if cfg!(windows) {
            self.windows_service_name()
        } else {
            self.systemd_unit_name()
        }
    }

    /// Full daemon detection for this identity: whether the process
    /// is a daemon and, when it is, the derived service label and
    /// system-versus-user mode.
    #[must_use]
    pub fn detect(&self, markers: &DaemonMarkers) -> DaemonState {
        crate::platform::detect_impl(self, markers)
    }
}
