mod generated;
pub use generated::*;
// DONT USE 6


//For Buttons
const GPIO_EXTI_TEMP: u8 = GPIO_EXTI as u8;
const GPIO_BTN_USER_TEMP: u8 = GPIO_INPUT | GPIO_FLOAT | GPIO_EXTI_TEMP | GPIO_PORTC | GPIO_PIN13;
pub const GPIO_BTN_USER: u32 = GPIO_BTN_USER_TEMP as u32;
