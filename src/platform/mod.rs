//! Per-platform detection back ends.

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub(crate) use unix::{
    detect_impl,
    is_daemon_impl,
};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use windows::{
    detect_impl,
    is_daemon_impl,
};

#[cfg(not(any(unix, windows)))]
mod fallback;
#[cfg(not(any(unix, windows)))]
pub(crate) use fallback::{
    detect_impl,
    is_daemon_impl,
};

/// Returns `true` if systemd is the active init system.
///
/// Uses the canonical `sd_booted(3)` check: the directory
/// `/run/systemd/system` exists only when systemd is PID 1.
/// Fast (a single `stat`), no process spawning.
#[cfg(target_os = "linux")]
#[must_use]
pub fn systemd_available() -> bool {
    std::path::Path::new("/run/systemd/system").exists()
}

/// Non-Linux platforms never use systemd.
#[cfg(not(target_os = "linux"))]
#[must_use]
pub fn systemd_available() -> bool {
    false
}
