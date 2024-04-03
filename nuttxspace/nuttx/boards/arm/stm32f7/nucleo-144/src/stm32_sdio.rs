/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_sdio.c
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
 //use lazy_static::lazy_static;

//DEFINE config first -- encompasses all
cfg_if::cfg_if! 
{ 
    if #[cfg(CONFIG_MMCSD)]
    {
        const HAVE_NCD: i32 = 1;
        //if macro defined, have constant
        if cfg!(not(GPIO_SDMMC1_NCD))
        {
            const HAVE_NCD: i32 = 0;
        }

        //define struct statically
        //https://users.rust-lang.org/t/static-struct-with-a-string-inside-a-module-for-singleton-pattern/37475
        //https://github.com/rust-lang-nursery/lazy-static.rs
        lazy_static!
        {
            static ref g_sdio_dev = &mut sdio_dev_s;
        }

        //if constant is defined
        if cfg!(HAVE_NCD)
        {
            lazy_static!
            {
                static bool g_sd_inserted;
            }
        }

        if cfg!(HAVE_NCD)
        {
            unsafe
            {
                lazy_static
                {
                    stm32_ncd_interrupt(irq: i32, context: *mut c_void) -> cty::cint
                    {
                        present: bool;

                        present = not(stm32_gpioread(GPIO_SDMMC1_NCD));
                        if(g_sdio_dev && (present != g_sd_inserted))
                        {
                            unsafe
                            {
                                sdio_mediachange(g_sdio_dev, present);
                                g_sd_inserted = present;
                            }
                        }

                        return OK;
                    }
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn stm32_sdio_initialize() -> cty::cint
        {
            ret: i32;

            //create a null ptr to replace NULL in C
            let mut null_ptr: *const u8 = 0 as *const u8;

            if cfg!(HAVE_NCD)
            {
                unsafe
                {
                    /* Configure the card detect GPIO */
                    stm32_configgpio(GPIO_SDMMC1_NCD);
                    /* Register an interrupt handler for the card detect pin */
                    stm32_gpiosetevent(GPIO_SDMMC1_NCD, true, true, true, stm32_ncd_interrupt, null_ptr);
                }
            }
            
            /* Mount the SDIO-based MMC/SD block driver */
            /* First, get an instance of the SDIO interface */
            unsafe
            {
                finfo("Initializing SDIO slot %d\n".as_ptr() as *const u8, SDIO_SLOTNO);    
            }

            g_sdio_dev = sdio_initialize(SDIO_SLOTNO);
            if !g_sdio_dev
            {
                ferr("ERROR: Failed to initialize SDIO slot %d\n".as_ptr() as *const u8, SDIO_SLOTNO);
                return -ENODEV;
            }

            /* Now bind the SDIO interface to the MMC/SD driver */
            unsafe
            {
                finfo("Bind SDIO to the MMC/SD driver, minor=%d\n".as_ptr() as *const u8, SDIO_MINOR);
            }

            ret = mmcsd_slotinitialize(SDIO_MINOR, g_sdio_dev);

            if ret != OK
            {
                ferr("ERROR: Failed to bind SDIO to the MMC/SD driver: %d\n".as_ptr() as *const u8, ret);
                return ret;
            }

            unsafe
            {
                finfo("Successfully bound SDIO to the MMC/SD driver\n".as_ptr() as *const u8);
            }

            if cfg!(HAVE_NCD)
            {
                g_sd_inserted = !stm32_gpioread(GPIO_SDMMC1_NCD);
                unsafe
                {
                    finfo("Card detect : %d\n".as_ptr() as *const u8, g_sd_inserted);
                    sdio_mediachange(g_sdio_dev, g_sd_inserted);
                }
            }
            else
            {
                /* Assume that the SD card is inserted.  What choice do we have? */
                unsafe
                {
                    sdio_mediachange(g_sdio_dev, true);
                }
            }
            return OK;
        }
    }
}