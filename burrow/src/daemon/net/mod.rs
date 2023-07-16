use super::*;
use serde::{Deserialize, Serialize};

#[cfg(target_family = "unix")]
mod unix;
#[cfg(all(target_family = "unix", not(feature = "systemd")))]
pub use unix::{listen, DaemonClient};

#[cfg(all(target_os = "linux", feature = "systemd"))]
mod systemd;
#[cfg(all(target_os = "linux", feature = "systemd"))]
pub use systemd::{listen, DaemonClient};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{listen, DaemonClient};

#[derive(Clone, Serialize, Deserialize)]
pub struct DaemonRequest {
    pub command: DaemonCommand,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DaemonResponse {
    //  Error types can't be serialized, so this is the second best option.
    result: std::result::Result<(), String>,
}
