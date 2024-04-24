/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_buttons.c
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
 
#![cfg(CONFIG_ARCH_BUTTONS)]

use crate::bindings::*;
// use crate::bindings::CONFIG_ARCH_BUTTONS;


/****************************************************************************
 * Included Files
 ****************************************************************************/

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: board_button_initialize
 *
 * Description:
 *   board_button_initialize() must be called to initialize button resources.
 *   After that, board_buttons() may be called to collect the current state
 *   of all buttons or board_button_irq() may be called to register button
 *   interrupt handlers.
 *
 ****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn board_button_initialize() -> cty::uint32_t {
    stm32_configgpio(GPIO_BTN_USER as u32);
    NUM_BUTTONS as u32
}

/****************************************************************************
 * Name: board_buttons
 ****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn board_buttons() -> cty::uint32_t {
    stm32_gpioread(GPIO_BTN_USER as u32) as u32
}

/****************************************************************************
 * Button support.
 *
 * Description:
 *   board_button_initialize() must be called to initialize button resources.
 *   After that, board_buttons() may be called to collect the current state
 *   of all buttons or board_button_irq() may be called to register button
 *   interrupt handlers.
 *
 *   After board_button_initialize() has been called, board_buttons() may be
 *   called to collect the state of all buttons.  board_buttons() returns an
 *   32-bit bit set with each bit associated with a button.  See the
 *   BUTTON_*_BIT definitions in board.h for the meaning of each bit.
 *
 *   board_button_irq() may be called to register an interrupt handler that
 *   will be called when a button is depressed or released. The ID value is a
 *   button enumeration value that uniquely identifies a button resource. See
 *   the BUTTON_* definitions in board.h for the meaning of enumeration
 *   value.
 *
 ****************************************************************************/

#[cfg(CONFIG_ARCH_IRQBUTTONS)]
#[no_mangle]
pub extern "C" fn board_button_irq(id: u8, irqhandler: xcpt_t, arg: *mut cty::c_void) -> cty::c_int {
    let mut ret = -(EINVAL as i32);

    if id == BUTTON_USER {
        unsafe {
            sleep(3);
            ret = stm32_gpiosetevent(GPIO_BTN_USER as u32, true, true, true, irqhandler, arg);
        }
    }

    return ret;
}
