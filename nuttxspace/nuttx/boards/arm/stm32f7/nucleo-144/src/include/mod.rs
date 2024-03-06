#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod bindings;
pub use bindings::*;

pub const GPIO_LD1: u32 = GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN0;
pub const GPIO_LD2: u32 = GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN7;
pub const GPIO_LD3: u32 = GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN14;

pub const GPIO_LED_GREEN: u32 = GPIO_LD1;
pub const GPIO_LED_BLUE: u32 = GPIO_LD2;
pub const GPIO_LED_RED: u32 = GPIO_LD3;


// used by boot.c
pub const CONFIG_STM32F7_OTGFS: 