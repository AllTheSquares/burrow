pub mod ensureroot;

use tun::TunInterface;

#[no_mangle]
pub extern "C" fn start() -> bool {
    ensureroot::ensure_root();
    matches!(TunInterface::new(), Ok(_iface))
}
