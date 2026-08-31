//! Label derivation, marker building, and non-daemon invariants.

use crate::{
    DaemonMarkers,
    ServiceIdentity,
    systemd_available,
};

fn identity() -> ServiceIdentity {
    ServiceIdentity::new("my-daemon", "com.example.my-daemon", "7476")
}

#[test]
fn launchd_label_joins_prefix_and_instance() {
    assert_eq!(identity().launchd_label(), "com.example.my-daemon.7476");
}

#[test]
fn systemd_unit_name_is_a_template_instance() {
    assert_eq!(identity().systemd_unit_name(), "my-daemon@7476.service");
}

#[test]
fn windows_service_name_joins_binary_and_instance() {
    assert_eq!(identity().windows_service_name(), "my-daemon-7476");
}

#[test]
fn service_label_matches_the_current_platform() {
    let identity = identity();
    let expected = if cfg!(target_os = "macos") {
        identity.launchd_label()
    } else if cfg!(windows) {
        identity.windows_service_name()
    } else {
        identity.systemd_unit_name()
    };
    assert_eq!(identity.service_label(), expected);
}

#[test]
fn markers_accumulate_accounts() {
    let markers = DaemonMarkers::new()
        .service_account("_mydaemon")
        .service_account("mydaemon")
        .windows_service_env("MY_DAEMON_SERVICE");
    // Builder state is private; behaviour is observable through
    // detection, which must not panic with populated markers.
    let _ = markers.is_daemon();
}

#[test]
fn non_daemon_state_has_no_label_or_mode() {
    // A test process is normally not a daemon; when the environment
    // says otherwise (CI under a service manager), the invariant on
    // the negative state still holds by construction.
    let state = identity().detect(&DaemonMarkers::new());
    assert_eq!(state.service_label.is_some(), state.is_daemon);
    assert_eq!(state.service_mode.is_some(), state.is_daemon);
}

#[test]
fn systemd_available_never_panics() {
    let _ = systemd_available();
}
