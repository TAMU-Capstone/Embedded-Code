#![no_std]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


mod bindings;

mod stm32_autoleds;
mod stm32_reset;
mod stm32_appinitialize;
<<<<<<< HEAD
mod stm32_adc;
mod stm32_gpio;
=======
//mod stm32_adc;
mod stm32_buttons;
mod stm32_userleds;
>>>>>>> d4b78a6dc54190914a80fdb30beb828d7c456051
