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
mod stm32_usb;
mod stm32_boot;
mod stm32_spi;
// mod stm32_bringup;
// mod stm32_adc;
mod stm32_appinitialize;
<<<<<<< HEAD
<<<<<<< HEAD
mod stm32_buttons;
mod stm32_userleds;
=======
>>>>>>> 522ac6ee (moved initialized varibale in main function of adc outside cfg to fix not found error)
mod stm32_adc;
=======
//mod stm32_adc;
>>>>>>> 49660a99b487bd5d28cb9d3e29b520d99685f0d9
mod stm32_gpio;
