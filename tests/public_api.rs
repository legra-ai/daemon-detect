//! Public-API integration test: validated service values and the derived
//! platform labels, from the shipped crate.

use daemon_detect::{
    BinaryName,
    DaemonMarkers,
    EnvVarName,
    InvalidServiceValue,
    LaunchdLabelPrefix,
    ServiceAccountName,
    ServiceIdentity,
};

#[test]
fn identity_derives_every_platform_label() -> Result<(), InvalidServiceValue> {
    let identity = ServiceIdentity::new(
        BinaryName::try_new("my-daemon")?,
        LaunchdLabelPrefix::try_new("com.example.my-daemon")?,
        7476_u16,
    );
    assert_eq!(identity.launchd_label(), "com.example.my-daemon.7476");
    assert_eq!(identity.systemd_unit_name(), "my-daemon@7476.service");
    assert_eq!(identity.windows_service_name(), "my-daemon-7476");
    Ok(())
}

#[test]
fn markers_detect_nothing_in_a_plain_test_process() -> Result<(), InvalidServiceValue> {
    let markers = DaemonMarkers::new()
        .service_account(ServiceAccountName::try_new("_mydaemon")?)
        .windows_service_env(EnvVarName::try_new("MY_DAEMON_SERVICE_UNSET_FOR_TEST")?);
    assert!(!markers.is_daemon());
    Ok(())
}

#[test]
fn malformed_values_are_rejected_at_construction() {
    assert!(ServiceAccountName::try_new("").is_err());
    assert!(EnvVarName::try_new("has space").is_err());
    assert!(BinaryName::try_new("../escape").is_err());
}
