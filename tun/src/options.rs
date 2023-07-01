use fehler::throws;
use serde::{Deserialize, Serialize};
use std::io::Error;

use super::TunInterface;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TunOptions {
    /// (Windows + Linux) Name the tun interface.
    #[serde(default)]
    pub(crate) name: Option<String>,
    /// (Linux) Don't include packet information.
    #[serde(default)]
    pub(crate) no_pi: Option<()>,
    /// (Linux) Avoid opening an existing persistant device.
    #[serde(default)]
    pub(crate) tun_excl: Option<()>,
}

impl TunOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn no_pi(mut self, enable: bool) {
        self.no_pi = enable.then_some(());
    }

    pub fn tun_excl(mut self, enable: bool) {
        self.tun_excl = enable.then_some(());
    }

    #[throws]
    pub fn open(self) -> TunInterface {
        TunInterface::new_with_options(self)?
    }
}
