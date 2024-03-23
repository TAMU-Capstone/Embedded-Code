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

/****************************************************************************
 * Included Files
 ****************************************************************************/
 use crate::include::*;
 

//used Rust's #[cfg()] attributes, to include or exclude code based on configuration options of the adc's
//In Rust, the #[cfg()] attribute is typically used with predefined configuration options or feature flags. These options are usually defined in the Cargo.toml file or as command-line arguments during compilation.
 

/* in mod.rs
pub const ADC1_NCHANNELS: usize = 4;
pub const G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];
pub const G_PINLIST: [u32; ADC1_NCHANNELS] = [GPIO_ADC1_IN3, GPIO_ADC1_IN4, GPIO_ADC1_IN10, GPIO_ADC1_IN13];
*/


// Skipped this portion for now undef excludes code based on configuration options of the adc 
// In Rust, the #[cfg()] attribute is typically used with predefined configuration options or feature flags.
// Need to  find where CONFIG_STM32F7_ADC code is defined so that it can be properly executed or excluded based on these conditional statements

/* Up to 3 ADC interfaces are supported 
#if STM32F7_NADC < 3
#  undef CONFIG_STM32F7_ADC3
#endif

#if STM32F7_NADC < 2
#  undef CONFIG_STM32F7_ADC2
#endif

#if STM32F7_NADC < 1
#  undef CONFIG_STM32F7_ADC1
#endif

#if defined(CONFIG_STM32F7_ADC1) || defined(CONFIG_STM32F7_ADC2) || defined(CONFIG_STM32F7_ADC3)
#ifndef CONFIG_STM32F7_ADC1
#  warning "Channel information only available for ADC1"
#endif
*/


/* The number of ADC channels in the conversion list */
// #define ADC1_NCHANNELS 4
const ADC1_NCHANNELS: usize = 4;

/****************************************************************************
 * Private Data
 ****************************************************************************/
/* Identifying number of each ADC channel: Variable Resistor.
 *
 * {1,  2,  3, 4,  5,  6, 7,  8,  9, 10, 11, 12, 13, 15};
*/
/*
    #ifdef CONFIG_STM32F7_ADC1
    static const uint8_t  g_chanlist[ADC1_NCHANNELS] =
    {
    3, 4, 10, 13
    };
*/
#[cfg(feature = "stm32f7_adc1")] // must define the feature in cargo.toml file others didnt do this so check this later #[cfg(ONFIG_STM32F7_ADC1)] <- format in other files
const G_CHANLIST: [u8; ADC1_NCHANNELS] = [3, 4, 10, 13];

/* Configurations of pins used byte each ADC channels
 *
 * {GPIO_ADC1_IN1,  GPIO_ADC1_IN2,  GPIO_ADC1_IN3,
 *  GPIO_ADC1_IN4,  GPIO_ADC1_IN5,  GPIO_ADC1_IN6,
 *  GPIO_ADC1_IN7,  GPIO_ADC1_IN8,  GPIO_ADC1_IN9,
 *  GPIO_ADC1_IN10, GPIO_ADC1_IN11, GPIO_ADC1_IN12,
 *  GPIO_ADC1_IN13, GPIO_ADC1_IN15};
 */

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
// these are the C function that need to be called in the Rust 
extern "C" {
    fn stm32_configgpio(pin: u32);
}

// returns a pointer to the adc_dev_s structure.
extern "C" {
    fn stm32_adc_initialize(adc_num: u8, chanlist: *const u8, channels: u8) -> *mut adc_dev_s;
}

extern "C" {
    fn adc_register(path: *const u8, adc: *mut adc_dev_s) -> i32;
}
/****************************************************************************
 * Name: stm32_adc_setup
 *
 * Description:
 *   Initialize ADC and register the ADC driver.
 *
 ****************************************************************************/

// main function 
// Other converted files seem to have skipped the outer main function ifdef wrapper so skipping for now check back on this later
// Original C function was void this will return ok if succesful or an i32 error code if their are errors  
#[no_mangle]
pub fn stm32_adc_setup() -> Result<(), i32> {

    /* 
        #ifdef CONFIG_STM32F7_ADC1
        static bool initialized = false;
        struct adc_dev_s *adc; //declares a pointer to the adc_dev_s struct (the struct definition is available elsewhere?)
        int ret;
        int i;
    */

    #[cfg(feature = "stm32f7_adc1")]
    static mut INITIALIZED: bool = false; 
    let mut adc: Option<&mut adc_dev_s >= None; //mutable reference to the adc_dev_s struct initialized to None 
    let mut ret: i32 = 0; // for use with integer error codes i32 is signed
    let mut i: usize = 0; // for use as a loop counter

    /* Check if we have already initialized */
    // accessing or modifying a static mutable variable requires an unsafe block because Rust's borrow checker cannot guarantee the safety of concurrent access to mutable statics at compile time
    if !unsafe { INITIALIZED } { 

        /* Configure the pins as analog inputs for the selected channels 
        // g_pinlist is an array that holds the pin configurations for each ADC channel.
        // called with the pin configuration value g_pinlist[i] to configure the pin as an analog input
        for (i = 0; i < ADC1_NCHANNELS; i++) { 
                stm32_configgpio(g_pinlist[i]); 
            }
        }
        */
        /* Configure the pins as analog inputs for the selected channels */
        for i in 0..ADC1_NCHANNELS {
            if G_PINLIST[i] != 0 {
                // requires an unsafe block because Rust cannot guarantee the safety of the C function at compile
                unsafe {
                    stm32_configgpio(G_PINLIST[i]);
                }
            }
        }

        /*  Call stm32_adcinitialize() to get an instance of the ADC interface 
            adc = stm32_adc_initialize(1, g_chanlist, ADC1_NCHANNELS);
            if (adc == NULL) {
                aerr("ERROR: Failed to get ADC interface\n");
                return -ENODEV;
            }
        */

        /* Call stm32_adcinitialize() to get an instance of the ADC interface  */
        // in Rust arrays do not automatically decay to pointers like they do in C use as_ptr() in rust
        // cast the usize value to u8 matching the type expected by the C function for the number of channels.
        let adc = unsafe {
            stm32_adc_initialize(1, G_CHANLIST.as_ptr(), ADC1_NCHANNELS as i32) // need to double check what values this function expects
        };

        if adc.is_null() {
            println!("ERROR: Failed to get ADC interface");
            return Err(-19); // -ENODEV is similiar to -19 in Rust
        }

        /* Register the ADC driver at "/dev/adc0" 
        ret = adc_register("/dev/adc0", adc);
        if (ret < 0)
            {
            aerr("ERROR: adc_register failed: %d\n", ret);
            return ret;
            }
        */

        /* Register the ADC driver at "/dev/adc0" */
        let path = "/dev/adc0".to_string(); // might need to use CString to allocate a string on the heap.
        let ret = unsafe {
            let c_path = path.as_ptr() as *const u8;
            adc_register(c_path, adc)
        };

        if ret < 0 {
            // Handle the error case
            println!("ERROR: adc_register failed: {}", ret);
            return Err(ret);
        }

        /* Now we are initialized */
        initialized = true;

    }
    else{
        return -libc::ENOSYS; //Make sure to add libc crate to Cargo.toml
    }
    // Returns OK 
    Ok(())
}