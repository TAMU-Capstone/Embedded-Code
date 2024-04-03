#![no_std]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


mod bindings;

<<<<<<< Updated upstream
mod stm32_autoleds;
mod stm32_reset;
mod stm32_usb;
mod stm32_boot;
mod stm32_spi;
mod stm32_bringup;
mod stm32_appinitialize;
mod stm32_buttons;
//pub mod stm32_adc;
=======
pub mod stm32_autoleds;
pub mod stm32_reset;
pub mod stm32_usb;
pub mod stm32_boot;
pub mod stm32_spi;
pub mod stm32_bringup;
pub mod stm32_appinitialize;
pub mod stm32_adc;
>>>>>>> Stashed changes
