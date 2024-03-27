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
/* in mod.rs
pub const ADC1_NCHANNELS: usize = 4;
pub const G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];
pub const G_PINLIST: [u32; ADC1_NCHANNELS] = [GPIO_ADC1_IN3, GPIO_ADC1_IN4, GPIO_ADC1_IN10, GPIO_ADC1_IN13];
pub const ADC1_NCHANNELS: [u32; ADC1_NCHANNELS] = [GPIO_ADC1_IN3, GPIO_ADC1_IN4, GPIO_ADC1_IN10, GPIO_ADC1_IN13];S
*/

/****************************************************************************
 * Included Files and Fsunctions
 ****************************************************************************/
use crate::bindings::*; 
use crate::bindings::ENOSYS;
use heapless::String;
/* Up to 3 ADC interfaces are supported */
#[cfg_if::cfg_if(not(STM32F7_NADC >= 3), then(not(CONFIG_STM32F7_ADC3)))]

#[cfg_if::cfg_if(not(STM32F7_NADC >= 2), then(not(CONFIG_STM32F7_ADC2)))]

#[cfg_if::cfg_if(not(STM32F7_NADC >= 1), then(not(CONFIG_STM32F7_ADC1)))]

#[cfg(any(CONFIG_STM32F7_ADC1, CONFIG_STM32F7_ADC2, CONFIG_STM32F7_ADC3))]

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

pub const ADC1_NCHANNELS: usize = 4;
pub const G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];

#[cfg(CONFIG_STM32F7_ADC1)] 
const G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];

const G_PINLIST: [u32; ADC1_NCHANNELS] = [
     GPIO_ADC1_IN3,
     GPIO_ADC1_IN4,
     GPIO_ADC1_IN10,
     GPIO_ADC1_IN13,
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
#[no_mangle]
pub fn stm32_adc_setup() -> Result<(), i32> {

    #[cfg(CONFIG_STM32F7_ADC1)]
    static mut INITIALIZED: bool = false;

    let mut adc: Option<&mut adc_dev_s >= None; //mutable reference to the adc_dev_s struct initialized to None 
    let mut ret: i32 = 0; // for use with integer error codes i32 is signed
    let mut i: usize = 0; // for use as a loop counter

    /* Check if we have already initialized */
    if !unsafe { INITIALIZED } { 
        /* Configure the pins as analog inputs for the selected channels */
        for i in 0..ADC1_NCHANNELS {
            if G_PINLIST[i] != 0 {
                // requires an unsafe block because Rust cannot guarantee the safety of the C function at compile
                unsafe {
                    stm32_configgpio(G_PINLIST[i]);
                }
            }
        }   

        /* Call stm32_adcinitialize() to get an instance of the ADC interface  */
        let adc = unsafe {
            stm32_adc_initialize(1, G_CHANLIST.as_ptr(), ADC1_NCHANNELS as i32)
        };

        if adc.is_null() {
            //println!("ERROR: Failed to get ADC interface");
            return Err(-19); // -ENODEV is similiar to -19 in Rust
        }

        /* Register the ADC driver at "/dev/adc0" */
        //let path = String::from("/dev/adc0");
        let mut path = String::<32>::new(); // Create a new String with a buffer size of 32 bytes
        path.push_str("/dev/adc0"); // Append the string "/dev/adc0" to the String
        let ret = unsafe { adc_register(path.as_ptr().cast::<u8>(), adc.unwrap()) };
        /*
        let ret = unsafe {
            let c_path = path.as_ptr() as *const u8;
            adc_register(c_path, adc)
        };
        */

        if ret < 0 {
            // Handle the error case
            //println!("ERROR: adc_register failed: {}", ret); // cant use println
            return Err(ret);
        }

        /* Now we are initialized */
        INITIALIZED = true;

    }
    else{
        return Err(ENOSYS as i32); //Make sure to add libc crate to Cargo.toml
    }
    
    Ok(())
}