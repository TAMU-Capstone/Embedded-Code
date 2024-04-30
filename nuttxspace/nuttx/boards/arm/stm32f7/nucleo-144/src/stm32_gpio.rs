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
// at the top to apply to the entire file similiar to how they are using ifdef
#![cfg(all(CONFIG_DEV_GPIO, not(CONFIG_GPIO_LOWER_HALF)))]

/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*;
use crate::info;
use crate::cty;
use core::ptr;

/****************************************************************************
* Private Types
****************************************************************************/

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default)]
struct stm32gpio_dev_s {
    gpio: gpio_dev_s,
    id: u8,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default)]
struct stm32gpint_dev_s {
    stm32gpio: stm32gpio_dev_s,
    callback: pin_interrupt_t,
}
/***************************************************************************
 * Private Function Prototypes
 ****************************************************************************/
// Not needed in Rust
/****************************************************************************
 * Private Data
 ****************************************************************************/
// check if should add const in rust
const GPIN_OPS: gpio_operations_s = gpio_operations_s {
    go_read: Some(gpin_read),
    go_write: None,
    go_attach: None,
    go_enable: None,
    go_setpintype: None
};

const GPOUT_OPS: gpio_operations_s = gpio_operations_s {
    go_read: Some(gpout_read),
    go_write: Some(gpout_write),
    go_attach: None,
    go_enable: None,
    go_setpintype: None
};

const GPINT_OPS: gpio_operations_s = gpio_operations_s {
    go_read: Some(gpint_read),
    go_write: None,
    go_attach: Some(gpint_attach),
    go_enable: Some(gpint_enable),
    go_setpintype: None
};

/* This array maps the GPIO pins used as INPUT */
#[cfg(BOARD_NGPIOIN)]
const G_GPIOINPUTS: [u32; BOARD_NGPIOIN as usize] = [
    GPIO_IN1 as u32,
    GPIO_IN2 as u32,
    GPIO_IN3 as u32,
    GPIO_IN4 as u32,
];

#[cfg(BOARD_NGPIOIN)]
static mut G_GPIN: [Option<stm32gpio_dev_s>; BOARD_NGPIOIN as usize] = [None; BOARD_NGPIOIN as usize];

/* This array maps the GPIO pins used as OUTPUT */
#[cfg(BOARD_NGPIOOUT)]
const G_GPIOOUTPUTS: &[u32] = &[
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
static mut G_GPOUT: [Option<stm32gpio_dev_s>; BOARD_NGPIOIN as usize] = [None; BOARD_NGPIOIN as usize];

/* This array maps the GPIO pins used as INTERRUPT INPUTS */
#[cfg(BOARD_NGPIOINT)]
const G_GPIOINTINPUTS: [u32; BOARD_NGPIOINT as usize] = [GPIO_INT1 as u32];

#[cfg(BOARD_NGPIOINT)]
static mut G_GPINT: [Option<stm32gpint_dev_s>; BOARD_NGPIOINT as usize] = [None; BOARD_NGPIOINT as usize];

/****************************************************************************
 * Private Functions
 ****************************************************************************/
#[no_mangle]
extern "C" fn stm32gpio_interrupt(
    _irq: i32,
    _context: *mut cty::c_void,
    arg: *mut cty::c_void,
) -> i32 {
    let stm32gpint = unsafe { &mut *(arg as *mut stm32gpint_dev_s) };

    debug_assert!(stm32gpint.callback.is_some(), "Callback is None");

    let callback = stm32gpint.callback.unwrap();
    info!("Interrupt! callback={:p}\n");

    #[allow(non_snake_case)]
    let gpio = &mut stm32gpint.stm32gpio.gpio;
    let id = stm32gpint.stm32gpio.id;
    unsafe { callback(gpio, id) };

    OK
}

#[no_mangle]
extern "C" fn gpin_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };

    debug_assert!(
        stm32gpio as *const _ as usize != 0 && value as *const _ as usize != 0,
        "Null pointer"
    );
    debug_assert!(stm32gpio.id < BOARD_NGPIOIN, "Invalid GPIO ID");

    info!("Reading...\n");
    unsafe {
        *value = stm32_gpioread(G_GPIOINPUTS[stm32gpio.id as usize]);
    }
    OK
}

#[no_mangle]
extern "C" fn gpout_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };

    debug_assert!(
        stm32gpio as *const _ as usize != 0 && value as *const _ as usize != 0,
        "Null pointer"
    );
    debug_assert!(stm32gpio.id < BOARD_NGPIOOUT, "Invalid GPIO ID");

    info!(b"Reading\n");
    unsafe {
        *value = stm32_gpioread(G_GPIOOUTPUTS[stm32gpio.id as usize]);
    }

    OK
}

#[no_mangle]
extern "C" fn gpout_write(dev: *mut gpio_dev_s, value: bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpio = unsafe { &mut *(dev as *mut stm32gpio_dev_s) };

    debug_assert!(stm32gpio as *const _ as usize != 0, "Null pointer");
    debug_assert!(stm32gpio.id < BOARD_NGPIOOUT, "Invalid GPIO ID");

    info!(b"Writing {}\n");
    unsafe {
        stm32_gpiowrite(G_GPIOOUTPUTS[stm32gpio.id as usize], value);
    }
    OK
}

#[no_mangle]
extern "C" fn gpint_read(dev: *mut gpio_dev_s, value: *mut bool) -> i32 {
    #[allow(non_snake_case)]
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    debug_assert!(
        stm32gpint as *const _ as usize != 0 && value as *const _ as usize != 0,
        "Null pointer"
    );
    debug_assert!(
        stm32gpint.stm32gpio.id < BOARD_NGPIOINT,
        "Invalid GPIO ID"
    );

    info!(b"Reading int pin...\n");

    unsafe {
        *value = stm32_gpioread(G_GPIOINTINPUTS[stm32gpint.stm32gpio.id as usize]);
    }
    OK
}
#[no_mangle]
extern "C" fn gpint_attach(dev: *mut gpio_dev_s, callback: pin_interrupt_t) -> i32 {
    // dev is a pointer to a struct of type gpio_dev_s
    // we need to cast dev to a pointer of type stm32gpint_dev_s
    // C code: struct stm32gpint_dev_s *stm32gpint = (struct stm32gpint_dev_s *)dev;
    //let stm32gpint = dev as *mut stm32gpint_dev_s;
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };

    info!(b"Attaching the callback\n");
    /* Make sure the interrupt is disabled */
    unsafe {
        stm32_gpiosetevent(
            G_GPIOINTINPUTS[stm32gpint.stm32gpio.id as usize],
            false,
            false,
            false,
            None,
            ptr::null_mut(),
        );
    }
    info!(b"Attach %p\n", callback.unwrap()); // most likely need to cast callback as something else
    stm32gpint.callback = callback;
    OK // return for gpint_attatch
}

#[no_mangle]
extern "C" fn gpint_enable(dev: *mut gpio_dev_s, enable: bool) -> i32 {
    // for enable
    let stm32gpint = unsafe { &mut *(dev as *mut stm32gpint_dev_s) };
    if enable && stm32gpint.callback.is_some() {
        info!(b"Enabling the interrupt\n");
            /* Configure the interrupt for rising edge */
        unsafe {
            stm32_gpiosetevent(
                G_GPIOINTINPUTS[stm32gpint.stm32gpio.id as usize],
                true,
                false,
                false,
                Some(stm32gpio_interrupt),
                (&mut G_GPINT[stm32gpint.stm32gpio.id as usize].unwrap() as *mut stm32gpint_dev_s) as *mut cty::c_void,
            );
        }
    } else {
        info!(b"Disable the interrupt\n");
        unsafe {
            stm32_gpiosetevent(
                G_GPIOINTINPUTS[stm32gpint.stm32gpio.id as usize],
                true,
                false,
                false,
                None,
                ptr::null_mut(),
            );
        }
    }
    OK
}

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
    let mut pincount = 0;

    #[cfg(BOARD_NGPIOIN)]
    for i in 0..BOARD_NGPIOIN as usize {
        unsafe {
            G_GPIN[i] = Some(stm32gpio_dev_s::default());
            // Setup and register the GPIO pin
            G_GPIN[i].unwrap().gpio.gp_pintype = gpio_pintype_e::GPIO_INPUT_PIN as u8;
            G_GPIN[i].unwrap().gpio.gp_ops = &GPIN_OPS;
            G_GPIN[i].unwrap().id = i as u8;
            gpio_pin_register(&mut G_GPIN[i].unwrap().gpio, pincount);

            // Configure the pin that will be used as input
            stm32_configgpio(G_GPIOINPUTS[i]);
        }
        pincount += 1;
    }

    #[cfg(BOARD_NGPIOOUT)]
    for i in 0..BOARD_NGPIOOUT as usize {
        unsafe {
            G_GPOUT[i] = Some(stm32gpio_dev_s::default());
            // Setup and register the GPIO pin
            G_GPOUT[i].unwrap().gpio.gp_pintype = gpio_pintype_e::GPIO_OUTPUT_PIN as u8;
            G_GPOUT[i].unwrap().gpio.gp_ops = &GPOUT_OPS;
            G_GPOUT[i].unwrap().id = i as u8;
            gpio_pin_register(&mut G_GPOUT[i].unwrap().gpio, pincount);

            // Configure the pin that will be used as output
            stm32_gpiowrite(G_GPIOOUTPUTS[i], false);
            stm32_configgpio(G_GPIOOUTPUTS[i]);
        }
        pincount += 1;
    }

    #[cfg(BOARD_NGPIOINT)]
    for i in 0..BOARD_NGPIOINT as usize {
        unsafe {
            G_GPINT[i] = Some(stm32gpint_dev_s::default());
            // Setup and register the GPIO pin
            G_GPINT[i].unwrap().stm32gpio.gpio.gp_pintype = gpio_pintype_e::GPIO_INTERRUPT_PIN as u8;
            G_GPINT[i].unwrap().stm32gpio.gpio.gp_ops = &GPINT_OPS;
            G_GPINT[i].unwrap().stm32gpio.id = i as u8;
            gpio_pin_register(&mut G_GPINT[i].unwrap().stm32gpio.gpio, pincount);

            // Configure the pin that will be used as interrupt input
            stm32_configgpio(G_GPIOINTINPUTS[i]);
        }
        pincount += 1;
    }
    OK // final return for stm32_gpio_initialize() function
}