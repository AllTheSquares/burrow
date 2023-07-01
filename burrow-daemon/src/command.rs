use serde::{Deserialize, Serialize};
use tun::TunOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Start(StartOptions),
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOptions {
    tun: TunOptions,
}
