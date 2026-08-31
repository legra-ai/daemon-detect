//! Label derivation, value validation, marker building, and
//! non-daemon invariants.

use crate::{
    BinaryName,
    DaemonMarkers,
    EnvVarName,
    LaunchdLabelPrefix,
    ServiceAccountName,
    ServiceIdentity,
    ServiceInstance,
    systemd_available,
};

fn identity() -> ServiceIdentity {
    ServiceIdentity::new(
        BinaryName::try_new("my-daemon").expect("valid binary name"),
        LaunchdLabelPrefix::try_new("com.example.my-daemon").expect("valid prefix"),
        7476_u16,
    )
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
fn account_names_accept_platform_conventions() {
    for name in ["_mydaemon", "mydaemon", "my-daemon", "my.daemon", "svc_01"] {
        assert!(ServiceAccountName::try_new(name).is_ok(), "{name}");
    }
}

#[test]
fn account_names_reject_separators_and_bounds() {
    for name in ["", "with space", "with:colon", "with/slash", "naïve"] {
        assert!(ServiceAccountName::try_new(name).is_err(), "{name:?}");
    }
    assert!(ServiceAccountName::try_new("a".repeat(33)).is_err());
    assert!(ServiceAccountName::try_new("a".repeat(32)).is_ok());
}

#[test]
fn env_var_names_reject_equals_and_bounds() {
    assert!(EnvVarName::try_new("MY_DAEMON_SERVICE").is_ok());
    for name in ["", "WITH=EQUALS", "with space", "é"] {
        assert!(EnvVarName::try_new(name).is_err(), "{name:?}");
    }
    assert!(EnvVarName::try_new("A".repeat(129)).is_err());
}

#[test]
fn binary_names_reject_paths_and_template_separators() {
    assert!(BinaryName::try_new("my-daemon.exe").is_ok());
    for name in ["", "dir/bin", "dir\\bin", "with@at", "with space"] {
        assert!(BinaryName::try_new(name).is_err(), "{name:?}");
    }
    assert!(BinaryName::try_new("a".repeat(65)).is_err());
}

#[test]
fn launchd_prefixes_must_be_reverse_dns_segments() {
    assert!(LaunchdLabelPrefix::try_new("com.example.my-daemon").is_ok());
    for prefix in [
        "",
        ".leading",
        "trailing.",
        "dou..ble",
        "seg ment",
        "com.exämple",
    ] {
        assert!(LaunchdLabelPrefix::try_new(prefix).is_err(), "{prefix:?}");
    }
}

#[test]
fn instances_reject_separators_and_accept_ports() {
    assert!(ServiceInstance::try_new("z6MkAbCd").is_ok());
    assert_eq!(ServiceInstance::from(7476_u16).as_str(), "7476");
    for instance in ["", "a/b", "a@b", "a b"] {
        assert!(ServiceInstance::try_new(instance).is_err(), "{instance:?}");
    }
}

#[test]
fn invalid_value_errors_name_the_kind_and_value() {
    let err = ServiceAccountName::try_new("with:colon").expect_err("must reject");
    let message = err.to_string();
    assert!(message.contains("service-account name"), "{message}");
    assert!(message.contains("with:colon"), "{message}");
}

#[test]
fn markers_accumulate_accounts() {
    let markers = DaemonMarkers::new()
        .service_account(ServiceAccountName::try_new("_mydaemon").expect("valid"))
        .service_account(ServiceAccountName::try_new("mydaemon").expect("valid"))
        .windows_service_env(EnvVarName::try_new("MY_DAEMON_SERVICE").expect("valid"));
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
