#![no_std]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


mod bindings;
mod macros;

mod stm32_autoleds;
mod stm32_reset;
mod stm32_usb;
mod stm32_boot;
mod stm32_spi;
mod stm32_bringup;
mod stm32_pwm;
mod stm32_sdio;
mod stm32_gpio;
mod stm32_adc;
mod stm32_appinitialize;
mod stm32_buttons;
mod stm32_userleds;
// mod stm32_composite;
mod stm32_romfs_initialize;
mod stm32_dma_alloc;
mod stm32_qencoder;