#![doc = include_str!("../README.md")]

mod identity;
mod markers;
mod platform;
mod state;

#[cfg(test)]
mod tests;

pub use identity::ServiceIdentity;
pub use markers::DaemonMarkers;
pub use platform::systemd_available;
pub use state::{
    DaemonServiceMode,
    DaemonState,
};
