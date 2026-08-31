//! Windows detection: the configured service environment variable.

use crate::identity::ServiceIdentity;
use crate::markers::DaemonMarkers;
use crate::state::{
    DaemonServiceMode,
    DaemonState,
};

/// Quick service check: the configured environment variable is set.
///
/// A more robust check would inspect the parent process, but an
/// injected environment variable is the simplest cross-version
/// approach; without one configured, detection reports false.
pub(crate) fn is_daemon_impl(markers: &DaemonMarkers) -> bool {
    markers
        .windows_service_env
        .as_ref()
        .is_some_and(|variable| std::env::var_os(variable.as_str()).is_some())
}

/// Full detection. Windows services are always system-scoped.
pub(crate) fn detect_impl(identity: &ServiceIdentity, markers: &DaemonMarkers) -> DaemonState {
    if !is_daemon_impl(markers) {
        return DaemonState::not_a_daemon();
    }
    DaemonState {
        is_daemon: true,
        service_label: Some(identity.service_label()),
        service_mode: Some(DaemonServiceMode::System),
    }
}
