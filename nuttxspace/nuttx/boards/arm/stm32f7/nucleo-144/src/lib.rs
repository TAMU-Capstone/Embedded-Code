#![no_std]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


pub mod include;
pub mod stm32_autoleds;
pub mod stm32_reset;
pub mod stm32_usb;
pub mod stm32_boot;
mod bindings;

mod stm32_autoleds;
mod stm32_reset;
mod stm32_appinitialize;
