/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_adc.c
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

#![cfg(CONFIG_ADC)]
#![cfg(any(CONFIG_STM32F7_ADC1, CONFIG_STM32F7_ADC2, CONFIG_STM32F7_ADC3))]

#[cfg(not(CONFIG_STM32F7_ADC1))]
compile_error!("Channel information only available for ADC1");


/****************************************************************************
 * Included Files and Fsunctions
 ****************************************************************************/
use crate::bindings::*;
use crate::{err, info};

// use crate::bindings::stm32_adc_initialize;

/* Up to 3 ADC interfaces are supported */


/* This only prints a warning it does not seem to exclude any code based on the condition so i am just going to take it out for now and add back if we get println working
mod adc_config {
    #[cfg(not(CONFIG_STM32F7_ADC1))]
    println!("Channel information only available for ADC1");
}
*/

/* The number of ADC channels in the conversion list */
const ADC1_NCHANNELS: usize = 4;

/****************************************************************************
 * Private Data
 ****************************************************************************/
/* Identifying number of each ADC channel: Variable Resistor.
 *
 * {1,  2,  3, 4,  5,  6, 7,  8,  9, 10, 11, 12, 13, 15};
*/

#[cfg(CONFIG_STM32F7_ADC1)]
static G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];

#[cfg(CONFIG_STM32F7_ADC1)]
const G_PINLIST: [u32; ADC1_NCHANNELS] = [
    GPIO_ADC1_IN3, GPIO_ADC1_IN4, GPIO_ADC1_IN10, GPIO_ADC1_IN13
];

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_adc_setup
 *
 * Description:
 *   Initialize ADC and register the ADC driver.
 *
 ****************************************************************************/

// main function
#[cfg(CONFIG_STM32F7_ADC1)]
#[no_mangle]
pub extern "C" fn stm32_adc_setup() -> i32 {
    use crate::err;
    use core::ptr;

    static mut INITIALIZED: bool = false;

    /* Check if we have already initialized */
    if unsafe { !INITIALIZED } {
        /* Configure the pins as analog inputs for the selected channels */
        G_PINLIST
            .iter()
            .filter(|p| **p != 0)
            .for_each(|p| unsafe { stm32_configgpio(*p); });


        let Some(adc) = ptr::NonNull::new( unsafe {
            stm32_adc_initialize(1, G_CHANLIST.as_ptr(), ADC1_NCHANNELS as i32)
        }) else {
            err!("ERROR: Failed to get ADC interface");
            return -(ENODEV as i32);
        };

        let ret = unsafe {
            adc_register(b"/dev/adc0" as *const u8, adc.as_ptr())
        };
        if ret != OK {
            err!("ERROR: adc_register failed: %d\n", ret);
            return ret;
        }
        /* Now we are initialized */
        unsafe {
            INITIALIZED = true;
        }
    }
    OK
}

#[cfg(not(CONFIG_STM32F7_ADC1))]
#[no_mangle]
pub extern "C" fn stm32_adc_setup() -> i32 {
    -(ENOSYS as i32)
}
