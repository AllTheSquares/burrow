use serde::{Deserialize, Serialize};
use tun::TunOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {
    Start(DaemonStartOptions),
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonStartOptions {
    tun: TunOptions,
}
