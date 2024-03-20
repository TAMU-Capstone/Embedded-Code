#![allow(dead_code)]

mod bindings;
pub use bindings::*;

pub const GPIO_LD1: u32 = GPIO_OUTPUT | GPIO_PUSHPULL as u32 | GPIO_SPEED_50MHz as u32 | GPIO_OUTPUT_CLEAR as u32 | GPIO_PORTB as u32 | GPIO_PIN0 as u32;
pub const GPIO_LD2: u32 = GPIO_OUTPUT | GPIO_PUSHPULL as u32 | GPIO_SPEED_50MHz as u32 | GPIO_OUTPUT_CLEAR as u32 | GPIO_PORTB as u32 | GPIO_PIN7 as u32;
pub const GPIO_LD3: u32 = GPIO_OUTPUT | GPIO_PUSHPULL as u32 | GPIO_SPEED_50MHz as u32 | GPIO_OUTPUT_CLEAR as u32 | GPIO_PORTB as u32 | GPIO_PIN14 as u32;

pub const GPIO_LED_GREEN: u32 = GPIO_LD1;
pub const GPIO_LED_BLUE: u32 = GPIO_LD2;
pub const GPIO_LED_RED: u32 = GPIO_LD3;
