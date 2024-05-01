#![no_std]
// #![cfg_attr(not(test), no_std)]


// Reference: https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


mod bindings;
mod macros;

// The following format is based off of Make.defs
// Defaults
mod stm32_bringup;
mod stm32_boot;

// Conditional Files
#[cfg(CONFIG_ARCH_LEDS)]
mod stm32_autoleds;
#[cfg(not(CONFIG_ARCH_LEDS))]
mod stm32_userleds;

#[cfg(CONFIG_ARCH_BUTTONS)]
mod stm32_buttons;

#[cfg(CONFIG_BOARDCTL)]
mod stm32_appinitialize;

#[cfg(CONFIG_DEV_GPIO)]
mod stm32_gpio;

#[cfg(CONFIG_SPI)]
mod stm32_spi;

#[cfg(CONFIG_ADC)]
mod stm32_adc;

#[cfg(CONFIG_PWM)]
mod stm32_pwm;

#[cfg(CONFIG_MMCSD)]
mod stm32_sdio;

#[cfg(CONFIG_STM32F7_OTGFS)]
mod stm32_usb;

#[cfg(CONFIG_STM32F7_BBSRAM)]
mod stm32_bbsram;

#[cfg(CONFIG_BOARDCTL_RESET)]
mod stm32_reset;

#[cfg(CONFIG_STM32_ROMFS)]
mod stm32_romfs_initialize;

#[cfg(CONFIG_SENSORS_QENCODER)]
mod stm32_qencoder;

#[cfg(all(CONFIG_STM32F7_CAN, CONFIG_STM32F7_CAN_CHARDRIVER))]
mod stm32_can;

#[cfg(all(CONFIG_STM32F7_CAN, CONFIG_STM32F7_CAN_SOCKET))]
mod stm32_cansock;

#[cfg(CONFIG_USBDEV_COMPOSITE)]
mod stm32_composite;
