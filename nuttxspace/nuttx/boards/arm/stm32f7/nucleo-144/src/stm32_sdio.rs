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
 
#![allow(unused_imports)]
#![cfg(CONFIG_MMCSD)]

use crate::bindings::*;
use core::ptr;

static mut G_SDIO_DEV: Option<ptr::NonNull<sdio_dev_s>> = None;

#[cfg(GPIO_SDMMC1_NCD)]
static mut G_SD_INSERTED: bool = false;


#[cfg(GPIO_SDMMC1_NCD)]
unsafe extern "C" fn stm32_ncd_interrupt(irq: i32, _context: *mut cty::c_void, _arg: *mut cty::c_void) -> cty::c_int {
    let present = !stm32_gpioread(GPIO_SDMMC1_NCD);
    if let Some(sdio_dev) = &mut G_SDIO_DEV {
        if present != G_SD_INSERTED {
            sdio_mediachange(sdio_dev.as_ptr(), present);
            G_SD_INSERTED = present;
        }
    }
    OK
}


#[no_mangle]
pub unsafe extern "C" fn stm32_sdio_initialize() -> cty::c_int
{
    #[cfg(GPIO_SDMMC1_NCD)]
    {
        stm32_configgpio(GPIO_SDMMC1_NCD);  // Configure the card detect GPIO
        stm32_gpiosetevent(                 // Register an interrupt handler for the card detect pin
            GPIO_SDMMC1_NCD,
            true,
            true,
            true,
            Some(stm32_ncd_interrupt),
            ptr::null_mut()
        ); 
    }

    /* Mount the SDIO-based MMC/SD block driver */
    /* First, get an instance of the SDIO interface */
    finfo("Initializing SDIO slot %d\n".as_ptr(), SDIO_SLOTNO);    
    G_SDIO_DEV = ptr::NonNull::new(sdio_initialize(SDIO_SLOTNO as i32));

    match G_SDIO_DEV {
        Some(_) => finfo("Bind SDIO to the MMC/SD driver, minor=%d\n".as_ptr(), SDIO_MINOR),
        None => {
            ferr("ERROR: Failed to initialize SDIO slot %d\n".as_ptr(), SDIO_SLOTNO);
            return -(ENODEV as i32);
        }
    }

    match mmcsd_slotinitialize(SDIO_MINOR, G_SDIO_DEV.unwrap().as_ptr()) {
        OK => finfo("Successfully bound SDIO to the MMC/SD driver\n".as_ptr()),
        ret => {
            ferr("ERROR: Failed to bind SDIO to the MMC/SD driver: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    #[cfg(GPIO_SDMMC1_NCD)] {
        G_SD_INSERTED = !stm32_gpioread(GPIO_SDMMC1_NCD);
        finfo!("Card detect : %d\n".as_ptr(), G_SD_INSERTED);
        sdio_mediachange(G_SDIO_DEV.unwrap().as_ptr(), G_SD_INSERTED);
    }
    #[cfg(not(GPIO_SDMMC1_NCD))] {
        sdio_mediachange(G_SDIO_DEV.unwrap().as_ptr(), true);
    }

    OK
}