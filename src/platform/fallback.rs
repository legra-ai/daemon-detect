//! Fallback for platforms that are neither Unix nor Windows.

use crate::identity::ServiceIdentity;
use crate::markers::DaemonMarkers;
use crate::state::DaemonState;

/// Unknown platforms never report daemon mode.
pub(crate) fn is_daemon_impl(_markers: &DaemonMarkers) -> bool {
    false
}

/// Unknown platforms report the not-a-daemon state.
pub(crate) fn detect_impl(_identity: &ServiceIdentity, _markers: &DaemonMarkers) -> DaemonState {
    DaemonState::not_a_daemon()
}
