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
use crate::bindings::stm32gpio_dev_s;
//use crate::bindings::BOARD_NGPIOIN; // Importing BOARD_NGPIOIN from another module

/****************************************************************************
* Private Types
****************************************************************************/

#[cfg(all(CONFIG_DEV_GPIO, not(CONFIG_GPIO_LOWER_HALF)))]
pub struct stm32gpio_dev_s {
    pub gpio: *mut gpio_dev_s,
    pub id: u8,
}
#[cfg(all(CONFIG_DEV_GPIO, not(CONFIG_GPIO_LOWER_HALF)))]
pub struct stm32gpint_dev_s {
    pub stm32gpio: *mut stm32gpio_dev_s,
    pub callback: pin_interrupt_t,
}

/****************************************************************************
 * Private Function Prototypes
 ****************************************************************************/
fn gpin_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32;
fn gpout_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32;
fn gpout_write(dev: &mut gpio_dev_s, value: bool) -> i32;
fn gpint_read(dev: &mut gpio_dev_s, value: &mut bool) -> i32;
fn gpint_attach(dev: &mut gpio_dev_s, callback: pin_interrupt_t) -> i32;
fn gpint_enable(dev: &mut gpio_dev_s, enable: bool) -> i32;

/****************************************************************************
 * Private Data
 ****************************************************************************/
// check if should add const in rust
const gpin_ops: gpio_operations_s = gpio_operations_s {
    go_read: gpin_read,
    go_write: NULL,
    go_attach: NULL,
    go_enable: NULL,
};

const gpout_ops: gpio_operations_s = gpio_operations_s {
    go_read: gpout_read,
    go_write: gpout_write,
    go_attach: NULL,
    go_enable: NULL,
};

const gpint_ops: gpio_operations_s = gpio_operations_s {
    go_read: gpint_read,
    go_write: NULL,
    go_attach: gpint_attach,
    go_enable: gpint_enable,
};

/* This array maps the GPIO pins used as INTERRUPT INPUTS */
const BOARD_NGPIOIN_GT_0: bool = BOARD_NGPIOIN > 0; // TAKE OUT LATER WHEN THIS IS IN SCOPE FROM COLES CHANGES
#[cfg(BOARD_NGPIOIN_GT_0)]
const G_GPIOINPUTS: [u32; BOARD_NGPIOIN as usize] = [
    GPIO_IN1,
    GPIO_IN2,
    GPIO_IN3,
    GPIO_IN4,
];

/* This array maps the GPIO pins used as INTERRUPT INPUTS */
#[cfg(BOARD_NGPIOIN_GT_0)]
// In C, you can have arrays of structs. Each element of the array is an instance of the struct type.
// static struct stm32gpio_dev_s g_gpin[BOARD_NGPIOIN];
static mut g_gpin: [*mut stm32gpio_dev_s; BOARD_NGPIOIN as usize] = [core::ptr::null_mut(); BOARD_NGPIOIN as usize];

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

// const BOARD_NGPIOIN_GT_0: bool = BOARD_NGPIOIN > 0; // TAKE OUT LATER WHEN THIS IS IN SCOPE FROM COLES CHANGES
//static struct stm32gpio_dev_s g_gpout[BOARD_NGPIOOUT];
static mut g_gpout: [*mut stm32gpio_dev_s; BOARD_NGPIOIN as usize] = [core::ptr::null_mut(); BOARD_NGPIOIN as usize];

