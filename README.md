# daemon-detect

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![CI][ci-badge]][ci-url]
[![License][license-badge]][license-url]
[![Downloads][downloads-badge]][downloads-url]

Detect whether the current process runs as an OS daemon — launchd,
systemd, or a Windows service — and derive its service label and scope.

A binary that can run both interactively and as a managed service often
needs to know which one it is *right now*: to route telemetry, pick config
paths, decide whether to daemonize helpers, or report its own service unit
for `systemctl` / `launchctl` tooling. `daemon-detect` answers that from
process metadata with no process spawning:

- **Unix** — parent PID 1 (launchd / systemd / init), or the effective UID
  matching a service account you configure.
- **Windows** — an environment variable your service wrapper injects.
- **systemd presence** — the canonical `sd_booted(3)` check (a single
  `stat` of `/run/systemd/system`).

## Usage

```rust
use daemon_detect::{
    BinaryName, DaemonMarkers, EnvVarName, LaunchdLabelPrefix, ServiceAccountName,
    ServiceIdentity, systemd_available,
};

# fn main() -> Result<(), daemon_detect::InvalidServiceValue> {
let markers = DaemonMarkers::new()
    .service_account(ServiceAccountName::try_new("_mydaemon")?) // macOS convention
    .service_account(ServiceAccountName::try_new("mydaemon")?) // Linux convention
    .windows_service_env(EnvVarName::try_new("MY_DAEMON_SERVICE")?);

// Quick check for early startup decisions:
let daemonized = markers.is_daemon();

// Full detection with a derived service label. Every value is
// validated at construction, so a malformed account name, label
// segment, or instance can never reach the OS surface:
let identity = ServiceIdentity::new(
    BinaryName::try_new("my-daemon")?,
    LaunchdLabelPrefix::try_new("com.example.my-daemon")?,
    7476_u16, // ports convert directly into a ServiceInstance
);
let state = identity.detect(&markers);
if state.is_daemon {
    println!(
        "running as {} ({:?})",
        state.service_label.as_deref().unwrap_or("?"),
        state.service_mode,
    );
}

// Label formats are also available directly, on every platform:
assert_eq!(identity.launchd_label(), "com.example.my-daemon.7476");
assert_eq!(identity.systemd_unit_name(), "my-daemon@7476.service");
assert_eq!(identity.windows_service_name(), "my-daemon-7476");

let _ = daemonized;
let _ = systemd_available();
# Ok(())
# }
```

`ServiceIdentity` carries the binary name (systemd and Windows labels), a
reverse-DNS launchd prefix, and an instance discriminator (a port, an
identity key, …) so one binary can run several service instances. Each is
a validated type — `BinaryName`, `LaunchdLabelPrefix`, `ServiceInstance`,
`ServiceAccountName`, `EnvVarName` — whose `try_new` enforces the limits
of the OS surface it is handed to (length bounds, ASCII-graphic content,
and the separator characters of passwd entries, environment blocks,
systemd template units, and launchd labels).
`DaemonState::service_mode` distinguishes system-scoped services (root or
a configured service account) from per-user ones (`launchctl` agents,
`systemd --user`).

The crate performs no I/O beyond a `stat` and passwd lookups, spawns
nothing, and has no dependencies outside `libc` on Unix.

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE));
- MIT License ([`LICENSE-MIT`](LICENSE-MIT)).

## Links

[crates-badge]: https://img.shields.io/crates/v/daemon-detect.svg
[crates-url]: https://crates.io/crates/daemon-detect
[docs-badge]: https://docs.rs/daemon-detect/badge.svg
[docs-url]: https://docs.rs/daemon-detect
[ci-badge]: https://github.com/legra-ai/daemon-detect/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/legra-ai/daemon-detect/actions/workflows/ci.yml
[license-badge]: https://img.shields.io/crates/l/daemon-detect.svg
[license-url]: https://github.com/legra-ai/daemon-detect/blob/main/LICENSE-APACHE
[downloads-badge]: https://img.shields.io/crates/d/daemon-detect.svg
[downloads-url]: https://crates.io/crates/daemon-detect
