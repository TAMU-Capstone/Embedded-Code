/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_userleds.c
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


/****************************************************************************
 * Private Data
****************************************************************************/

/* This array maps an LED number to GPIO pin configuration and is indexed by
* BOARD_LED_<color>
*/

static g_ledcfg: [u32; BOARD_NLEDS as usize] = [
    GPIO_LED_GREEN,
    GPIO_LED_BLUE,
    GPIO_LED_RED,
];

/****************************************************************************
 * Public Functions
****************************************************************************/

/****************************************************************************
 * Name: board_userled_initialize
*
* Description:
*   If CONFIG_ARCH_LEDS is defined, then NuttX will control the on-board
*   LEDs.  If CONFIG_ARCH_LEDS is not defined, then the
*   board_userled_initialize() is available to initialize the LED from user
*   application logic.
*
****************************************************************************/

#[no_mangle]
pub extern "C" fn board_userled_initialize() -> cty::uint32_t {
    let i: i32;

    for &gpio in &g_ledcfg {
        unsafe {
            stm32_configgpio(gpio);
        }
    }

    return BOARD_NLEDS.into()

}


/****************************************************************************
 * Name: board_userled
*
* Description:
*   If CONFIG_ARCH_LEDS is defined, then NuttX will control the on-board
*  LEDs.  If CONFIG_ARCH_LEDS is not defined, then the board_userled() is
*  available to control the LED from user application logic.
*
****************************************************************************/

#[no_mangle]
pub extern "C" fn  board_userled(int led, bool ledon) {

}


/****************************************************************************
 * Name: board_userled_all
*
* Description:
*   If CONFIG_ARCH_LEDS is defined, then NuttX will control the on-board
*  LEDs.  If CONFIG_ARCH_LEDS is not defined, then the board_userled_all()
*  is available to control the LED from user application logic. NOTE: since
*  there is only a single LED on-board, this is function is not very useful.
*
****************************************************************************/


