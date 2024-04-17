/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_autoleds.c
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
#![cfg(CONFIG_ARCH_LEDS)]

use crate::bindings::*;

/****************************************************************************
 * Private Data
 ****************************************************************************/

/* Indexed by BOARD_LED_<color> */

static G_LEDMAP: [u32; BOARD_NLEDS as usize] = [
    GPIO_LED_GREEN,
    GPIO_LED_BLUE,
    GPIO_LED_RED
];

static mut G_INITIALIZED: bool = false;
/****************************************************************************
 * Private Functions
 ****************************************************************************/

// extern "C" {
//     fn stm32_gpiowrite(pin: u32, state: bool);
//     fn stm32_configgpio(pin: u32);
// }

#[no_mangle]
pub extern "C" fn phy_set_led(led: u8, state: bool) {
    unsafe {
        stm32_gpiowrite(G_LEDMAP[led as usize], state);
    }
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: board_autoled_initialize
 ****************************************************************************/
#[no_mangle]
pub extern "C" fn board_autoled_initialize() {
    // G_LEDMAP.map(|pin| unsafe { stm32_configgpio(pin) });
    
    for &pin in G_LEDMAP.iter() {
        unsafe {
            stm32_configgpio(pin);
        }
    }
}

/****************************************************************************
 * Name: board_autoled_on
 ****************************************************************************/
#[no_mangle]
pub extern "C" fn board_autoled_on(led: u8) {
    match led {
        LED_SIGNAL => phy_set_led(BOARD_LED_GREEN, true),
        LED_PANIC | LED_IDLE =>  phy_set_led(BOARD_LED_RED, true),
        LED_HEAPALLOCATE | LED_INIRQ => phy_set_led(BOARD_LED_BLUE, true),
        LED_IRQSENABLED => {
            phy_set_led(BOARD_LED_BLUE, false);
            phy_set_led(BOARD_LED_GREEN, true);
        }
        LED_STACKCREATED => {
            phy_set_led(BOARD_LED_GREEN, true);
            phy_set_led(BOARD_LED_BLUE, true);
            unsafe { G_INITIALIZED = true; }
        }
        LED_ASSERTION => {
            phy_set_led(BOARD_LED_RED, true);
            phy_set_led(BOARD_LED_BLUE, true);
        }
        _ => {}
    }
}

/****************************************************************************
 * Name: board_autoled_off
 ****************************************************************************/

#[no_mangle]
pub extern "C" fn board_autoled_off(led: u8) {
    match led {
        LED_SIGNAL => phy_set_led(BOARD_LED_GREEN, false),
        LED_INIRQ => phy_set_led(BOARD_LED_BLUE, false),
        LED_ASSERTION => {
            phy_set_led(BOARD_LED_RED, false);
            phy_set_led(BOARD_LED_BLUE, false);
        }
        LED_PANIC | LED_IDLE => phy_set_led(BOARD_LED_RED, false),
        _ => {}
    }
}
