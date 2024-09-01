//! The panic handler

use core::panic::PanicInfo;
use log::*;

use crate::shutdown;

pub fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().unwrap());
    }

    shutdown(true)
}
