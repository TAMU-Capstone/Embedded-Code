/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_gpio.c
 *
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.  The
 * ASF licenses this file to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance with the
 * License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
 * License for the specific language governing permissions and limitations
 * under the License.
 *
 ****************************************************************************/

/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*; 

/* structs etc needed from other files
gpio_dev_s
stm32gpio_dev_s
pin_interrupt_t
gpio_operations_s
*/

/****************************************************************************
* Private Types
****************************************************************************/

#[cfg(all(CONFIG_DEV_GPIO, not(CONFIG_GPIO_LOWER_HALF)))]
pub struct stm32gpio_dev_s {
    pub gpio: gpio_dev_s,
    pub id: u8,
}
#[cfg(all(CONFIG_DEV_GPIO, not(CONFIG_GPIO_LOWER_HALF)))]
pub struct stm32gpint_dev_s {
    pub stm32gpio: stm32gpio_dev_s,
    pub callback: pin_interrupt_t,
}
/***************************************************************************
 * Private Function Prototypes
 ****************************************************************************/
/*
 fn gpin_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32; // MAY BE ABLE TO REMOVE THESE 
fn gpout_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32;
fn gpout_write(dev: &mut gpio_dev_s, value: bool) -> i32;
fn gpint_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32;
fn gpint_attach(dev: &mut gpio_dev_s, callback: pin_interrupt_t) -> i32;
fn gpint_enable(dev: &mut gpio_dev_s, enable: bool) -> i32;
*/
/****************************************************************************
 * Private Data
 ****************************************************************************/
// check if should add const in rust
const gpin_ops: gpio_operations_s  = gpio_operations_s {
    go_read: Some(gpin_read),
    go_write: None,
    go_attach: None,
    go_enable: None,
};

const gpout_ops: gpio_operations_s = gpio_operations_s {
    go_read: Some(gpout_read),
    go_write: Some(gpout_write),
    go_attach: None,
    go_enable: None,
};

const gpint_ops: gpio_operations_s = gpio_operations_s {
    go_read: Some(gpint_read),
    go_write: None,
    go_attach: Some(gpint_attach),
    go_enable: Some(gpint_enable),
};

/* This array maps the GPIO pins used as INPUT */
const BOARD_NGPIOIN_G0: bool = BOARD_NGPIOIN > 0;
#[cfg(BOARD_NGPIOIN_G0)]
const G_GPIOINPUTS: [u32; BOARD_NGPIOIN as usize] = [
    GPIO_IN1,
    GPIO_IN2,
    GPIO_IN3,
    GPIO_IN4,
];
#[cfg(BOARD_NGPIOIN)]
static mut g_gpin: [stm32gpio_dev_s; BOARD_NGPIOIN as usize] = [stm32gpio_dev_s::default(); BOARD_NGPIOIN as usize];

/* This array maps the GPIO pins used as OUTPUT */
#[cfg(BOARD_NGPIOOUT)]
const g_gpiooutputs: &[u32] = &[
    GPIO_LD1,
    GPIO_LD2,
    GPIO_LD3,
    GPIO_OUT1,
    GPIO_OUT2,
    GPIO_OUT3,
    GPIO_OUT4,
    GPIO_OUT5,
    #[cfg(not(CONFIG_STM32F7_TIM1_CH1NOUT))]
    GPIO_OUT6,
    #[cfg(not(CONFIG_STM32F7_TIM1_CH2NOUT))]
    GPIO_OUT7,
];
#[cfg(BOARD_NGPIOOUT)]
static mut g_gpout: [stm32gpio_dev_s; BOARD_NGPIOIN as usize] = [stm32gpio_dev_s::default(); BOARD_NGPIOIN as usize];

/* This array maps the GPIO pins used as INTERRUPT INPUTS */
const BOARD_NGPIOINT_G0: bool = BOARD_NGPIOINT > 0;
#[cfg(BOARD_NGPIOINT_G0)]
const g_gpiointinputs: [u32; BOARD_NGPIOINT] = [GPIO_INT1];

#[cfg(BOARD_NGPIOINT_G0)]
static mut g_gpint: [stm32gpint_dev_s; BOARD_NGPIOINT as usize] = [stm32gpint_dev_s::default(); BOARD_NGPIOINT as usize];

/****************************************************************************
 * Private Functions
 ****************************************************************************/
 #[no_mangle]
 fn stm32gpio_interrupt(irq: i32, context: *mut (), arg: *mut ()) -> i32 {
     #[allow(non_snake_case)]
     let stm32gpint = unsafe { &mut *(arg as *mut stm32gpint_dev_s) };
 
     debug_assert!(stm32gpint.callback.is_some(), "Callback is None");
 
     let callback = stm32gpint.callback.unwrap();
     gpioinfo!("Interrupt! callback={:p}\n", callback as *const ());
 
     #[allow(non_snake_case)]
     let gpio = &mut stm32gpint.stm32gpio.gpio;
     let id = stm32gpint.stm32gpio.id;
     callback(gpio, id);
 
     0
 }

 #[no_mangle]
 fn gpin_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
     #[allow(non_snake_case)]
     let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };
 
     debug_assert!(stm32gpio as *const _ as usize != 0 && value as *const _ as usize != 0, "Null pointer");
     debug_assert!((stm32gpio.id as usize) < BOARD_NGPIOIN, "Invalid GPIO ID");
 
     gpioinfo!("Reading...\n");
 
     unsafe {
         *value = stm32_gpioread(g_gpioinputs[stm32gpio.id as usize]);
     }
     0
 }

 #[no_mangle]
fn gpout_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };

    debug_assert!(stm32gpio as *const _ as usize != 0 && value as *const _ as usize != 0, "Null pointer");
    debug_assert!((stm32gpio.id as usize) < BOARD_NGPIOOUT, "Invalid GPIO ID");

    gpioinfo!("Reading...\n");

    unsafe {
        *value = stm32_gpioread(g_gpiooutputs[stm32gpio.id as usize]);
    }

    0
}

#[no_mangle]
fn gpout_write(dev: *mut gpio_dev_s, value: bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };

    debug_assert!(stm32gpio as *const _ as usize != 0, "Null pointer");
    debug_assert!((stm32gpio.id as usize) < BOARD_NGPIOOUT, "Invalid GPIO ID");

    gpioinfo!("Writing {}\n", value as i32);

    unsafe {
        stm32_gpiowrite(g_gpiooutputs[stm32gpio.id as usize], value);
    }

    0
}

#[no_mangle]
fn gpint_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    debug_assert!(stm32gpint as *const _ as usize != 0 && value as *const _ as usize != 0, "Null pointer");
    debug_assert!((stm32gpint.stm32gpio.id as usize) < BOARD_NGPIOINT, "Invalid GPIO ID");

    gpioinfo!("Reading int pin...\n");

    unsafe {
        *value = stm32_gpioread(g_gpiointinputs[stm32gpint.stm32gpio.id as usize]);
    }

    0
}

/*
#[no_mangle]
fn gpint_attach(dev: *mut gpio_dev_s, callback: Option<extern "C" fn(i32, *mut (), *mut ()) -> i32>) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    gpioinfo!("Attaching the callback\n");

    // Make sure the interrupt is disabled
    unsafe {
        stm32_gpiosetevent(
            g_gpiointinputs[stm32gpint.stm32gpio.id as usize],
            false,
            false,
            false,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }

    gpioinfo!("Attach {:?}\n", callback);

    stm32gpint.callback = callback;

    0
}
*/

/* Having trouble with the last 2 arguments into stm32_gpiosetevent since it expects null pointers 
#[no_mangle]
fn gpint_attach(dev: *mut gpio_dev_s, callback: Option<extern "C" fn(i32, *mut (), *mut ()) -> i32>) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    gpioinfo!("Attaching the callback\n");

    // Make sure the interrupt is disabled
    unsafe {
        stm32_gpiosetevent(
            g_gpiointinputs[stm32gpint.stm32gpio.id as usize],
            false,
            false,
            false,
            0 as *mut _,
            0 as *mut _,
        );
    }

    gpioinfo!("Attach {:?}\n", callback);

    stm32gpint.callback = callback;

    0
}

#[no_mangle]
fn gpint_enable(dev: *mut gpio_dev_s, enable: bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    if enable {
        if let Some(callback) = stm32gpint.callback {
            gpioinfo!("Enabling the interrupt\n");

            // Configure the interrupt for rising edge
            unsafe {
                stm32_gpiosetevent(
                    g_gpiointinputs[stm32gpint.stm32gpio.id as usize],
                    true,
                    false,
                    false,
                    callback,
                    &mut g_gpint[stm32gpint.stm32gpio.id as usize] as *mut _ as *mut (),
                );
            }
        }
    } else {
        gpioinfo!("Disable the interrupt\n");

        unsafe {
            stm32_gpiosetevent(
                g_gpiointinputs[stm32gpint.stm32gpio.id as usize],
                false,
                false,
                false,
                None,
                0 as *mut _,
            );
        }
    }

    0
}
*/

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_gpio_initialize
 *
 * Description:
 *   Initialize GPIO drivers for use with /apps/examples/gpio
 *
 ****************************************************************************/
 
#[no_mangle]
pub fn stm32_gpio_initialize() -> i32 {
    let mut i  = 0;
    let mut pincount = 0;
 
     #[cfg(BOARD_NGPIOIN_G0)] 
     fn setup_gpio_in() {
         for i in 0..BOARD_NGPIOIN as usize {
             // Setup and register the GPIO pin
             g_gpin[i].gpio.gp_pintype = GPIO_INPUT_PIN;
             g_gpin[i].gpio.gp_ops = &GPIN_OPS;
             g_gpin[i].id = i as u8;
             gpio_pin_register(&mut g_gpin[i].gpio, pincount);
 
             // Configure the pin that will be used as input
             stm32_configgpio(g_gpioinputs[i]);
             pincount += 1;
         }
     }

    pub const BOARD_NGPIOOUT_G0: bool = BOARD_NGPIOOUT > 0;
    #[cfg(BOARD_NGPIOOUT_G0)]
    fn setup_gpio_outputs() {
        for i in 0..BOARD_NGPIOOUT as usize {
            // Setup and register the GPIO pin
            g_gpout[i].gpio.gp_pintype = GPIO_OUTPUT_PIN;
            g_gpout[i].gpio.gp_ops = &GPOUT_OPS;
            g_gpout[i].id = i as u8;
            gpio_pin_register(&mut g_gpout[i].gpio, pincount);
    
            // Configure the pin that will be used as output
            stm32_gpiowrite(g_gpiooutputs[i], 0);
            stm32_configgpio(g_gpiooutputs[i]);
            pincount += 1;
        }
    }

    #[cfg(BOARD_NGPIOINT_G0)]
    fn setup_gpio_interrupts() {
        for i in 0..BOARD_NGPIOINT {
            // Setup and register the GPIO pin
            g_gpint[i as usize].stm32gpio.gpio.gp_pintype = GPIO_INTERRUPT_PIN;
            g_gpint[i as usize].stm32gpio.gpio.gp_ops = &gpint_ops;
            g_gpint[i as usize].stm32gpio.id = i;
            gpio_pin_register(&mut g_gpint[i as usize].stm32gpio.gpio, pincount);

            // Configure the pin that will be used as interrupt input
            stm32_configgpio(g_gpiointinputs[i as usize]);
            pincount += 1;
        }
    }
    0
}