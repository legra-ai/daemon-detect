//! Unix detection: parent PID 1 or a configured service-account UID.

use std::ffi::CString;

use crate::identity::ServiceIdentity;
use crate::markers::DaemonMarkers;
use crate::state::{
    DaemonServiceMode,
    DaemonState,
};

/// Quick daemon check: parent PID 1 or a service-account UID.
pub(crate) fn is_daemon_impl(markers: &DaemonMarkers) -> bool {
    // SAFETY: getppid never fails and touches no memory.
    let ppid = unsafe { libc::getppid() };
    if ppid == 1 {
        return true;
    }
    is_service_account_uid(markers)
}

/// Full detection with the derived service label and mode.
pub(crate) fn detect_impl(identity: &ServiceIdentity, markers: &DaemonMarkers) -> DaemonState {
    if !is_daemon_impl(markers) {
        return DaemonState::not_a_daemon();
    }
    // SAFETY: geteuid never fails and touches no memory.
    let euid = unsafe { libc::geteuid() };
    let mode = if euid == 0 || is_service_account_uid(markers) {
        DaemonServiceMode::System
    } else {
        DaemonServiceMode::User
    };
    DaemonState {
        is_daemon: true,
        service_label: Some(identity.service_label()),
        service_mode: Some(mode),
    }
}

/// Whether the effective UID belongs to one of the configured
/// service accounts. Root does not count as a service account by
/// itself.
fn is_service_account_uid(markers: &DaemonMarkers) -> bool {
    // SAFETY: geteuid never fails and touches no memory.
    let euid = unsafe { libc::geteuid() };
    if euid == 0 {
        return false;
    }
    for name in &markers.service_accounts {
        let Ok(c_name) = CString::new(name.as_str()) else {
            continue;
        };
        // SAFETY: c_name is a valid NUL-terminated string; getpwnam
        // returns a pointer into static storage or NULL.
        let pw = unsafe { libc::getpwnam(c_name.as_ptr()) };
        if !pw.is_null() && unsafe { (*pw).pw_uid } == euid {
            return true;
        }
    }
    false
}
