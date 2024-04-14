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
 // #[cfg(CONFIG_MMCSD)]
mod stm32_sdio {

    use core::ptr::NonNull;
    use crate::bindings::*;


    static mut G_SDIO_DEV: Option<NonNull<sdio_dev_s>> = None;

    // #[cfg(GPIO_SDMMC1_NCD)]
    static mut G_SD_INSERTED: bool = false;
    
    // #[cfg(GPIO_SDMMC1_NCD)]
    fn stm32_ncd_interrupt(irq: i32, _context: *mut cty::c_void) -> cty::c_int {
        unsafe {
            let present = !stm32_gpioread(GPIO_SDMMC1_NCD);
            if let Some(g_sdio) = &mut G_SDIO_DEV {
                if present != G_SD_INSERTED {
                    sdio_mediachange(g_sdio.as_ptr(), present);
                    G_SD_INSERTED = present;
                }
            }
        }
        OK
    }

    #[no_mangle]
    pub extern "C" fn stm32_sdio_initialize() -> cty::c_int
    {
        let ret: i32;


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

        G_SDIO_DEV = sdio_initialize(SDIO_SLOTNO);
        if !G_SDIO_DEV
        {
            ferr("ERROR: Failed to initialize SDIO slot %d\n".as_ptr() as *const u8, SDIO_SLOTNO);
            return -ENODEV;
        }

        /* Now bind the SDIO interface to the MMC/SD driver */
        unsafe
        {
            finfo("Bind SDIO to the MMC/SD driver, minor=%d\n".as_ptr() as *const u8, SDIO_MINOR);
        }

        ret = mmcsd_slotinitialize(SDIO_MINOR, G_SDIO_DEV);

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
            G_SD_INSERTED = !stm32_gpioread(GPIO_SDMMC1_NCD);
            unsafe
            {
                finfo("Card detect : %d\n".as_ptr() as *const u8, G_SD_INSERTED);
                sdio_mediachange(G_SDIO_DEV, G_SD_INSERTED);
            }
        }
        else
        {
            /* Assume that the SD card is inserted.  What choice do we have? */
            unsafe
            {
                sdio_mediachange(G_SDIO_DEV, true);
            }
        }
        return OK;
    }
}