#![no_std]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod stm32_autoleds;
pub mod stm32_reset;
pub mod stm32_usb;
pub mod stm32_boot;
pub mod bindings;
pub mod stm32_spi;
pub mod stm32_appinitialize;
