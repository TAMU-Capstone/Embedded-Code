/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_spi.c
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
#![cfg(CONFIG_SPI)]
#![allow(unused_imports)]

use crate::bindings::*;
use core::ptr;
use crate::{err, info};
/****************************************************************************
 * Pre-processor Definitions
****************************************************************************/

#[cfg(CONFIG_NUCLEO_SPI1_TEST)]
pub const NUCLEO_SPI1_TEST_MODE: u32 = match () {
    #[cfg(CONFIG_NUCLEO_SPI1_TEST_MODE0)]
    () => spi_mode_e_SPIDEV_MODE0,
    #[cfg(CONFIG_NUCLEO_SPI1_TEST_MODE1)]
    () => spi_mode_e_SPIDEV_MODE1,
    #[cfg(CONFIG_NUCLEO_SPI1_TEST_MODE2)]
    () => spi_mode_e_SPIDEV_MODE2,
    #[cfg(CONFIG_NUCLEO_SPI1_TEST_MODE3)]
    () => spi_mode_e_SPIDEV_MODE3,
    #[cfg(not(any(CONFIG_NUCLEO_SPI1_TEST_MODE0, CONFIG_NUCLEO_SPI1_TEST_MODE1, CONFIG_NUCLEO_SPI1_TEST_MODE2, CONFIG_NUCLEO_SPI1_TEST_MODE3)))]
    _ => compile_error!("No CONFIG_NUCLEO_SPI1_TEST_MODEx defined")
};

#[cfg(CONFIG_NUCLEO_SPI2_TEST)]
pub const NUCLEO_SPI2_TEST_MODE: u32 = match () {
    #[cfg(CONFIG_NUCLEO_SPI2_TEST_MODE0)]
    () => spi_mode_e_SPIDEV_MODE0,
    #[cfg(CONFIG_NUCLEO_SPI2_TEST_MODE1)]
    () => spi_mode_e_SPIDEV_MODE1,
    #[cfg(CONFIG_NUCLEO_SPI2_TEST_MODE2)]
    () => spi_mode_e_SPIDEV_MODE2,
    #[cfg(CONFIG_NUCLEO_SPI2_TEST_MODE3)]
    () => spi_mode_e_SPIDEV_MODE3,
    #[cfg(not(any(CONFIG_NUCLEO_SPI2_TEST_MODE0, CONFIG_NUCLEO_SPI2_TEST_MODE1, CONFIG_NUCLEO_SPI2_TEST_MODE2, CONFIG_NUCLEO_SPI2_TEST_MODE3)))]
    _ => compile_error!("No CONFIG_NUCLEO_SPI2_TEST_MODEx defined")
};

#[cfg(CONFIG_NUCLEO_SPI3_TEST)]
pub const NUCLEO_SPI3_TEST_MODE: u32 = match () {
    #[cfg(CONFIG_NUCLEO_SPI3_TEST_MODE0)]
    () => spi_mode_e_SPIDEV_MODE0,
    #[cfg(CONFIG_NUCLEO_SPI3_TEST_MODE1)]
    () => spi_mode_e_SPIDEV_MODE1,
    #[cfg(CONFIG_NUCLEO_SPI3_TEST_MODE2)]
    () => spi_mode_e_SPIDEV_MODE2,
    #[cfg(CONFIG_NUCLEO_SPI3_TEST_MODE3)]
    () => spi_mode_e_SPIDEV_MODE3,
    #[cfg(not(any(CONFIG_NUCLEO_SPI3_TEST_MODE0, CONFIG_NUCLEO_SPI3_TEST_MODE1, CONFIG_NUCLEO_SPI3_TEST_MODE2, CONFIG_NUCLEO_SPI3_TEST_MODE3)))]
    _ => compile_error!("No CONFIG_NUCLEO_SPI3_TEST_MODEx defined")
};

/****************************************************************************
 * Private Data
****************************************************************************/

#[cfg(CONFIG_STM32F7_SPI1)]
const G_SPI1GPIO: [u32; 4] = [
    #[cfg(GPIO_SPI1_CS0)] GPIO_SPI1_CS0,
    #[cfg(not(GPIO_SPI1_CS0))] 0,

    #[cfg(GPIO_SPI1_CS1)] GPIO_SPI1_CS1,
    #[cfg(not(GPIO_SPI1_CS1))] 0,

    #[cfg(GPIO_SPI1_CS2)] GPIO_SPI1_CS2,
    #[cfg(not(GPIO_SPI1_CS2))] 0,

    #[cfg(GPIO_SPI1_CS3)] GPIO_SPI1_CS3,
    #[cfg(not(GPIO_SPI1_CS3))] 0,
];

#[cfg(CONFIG_STM32F7_SPI2)]
const G_SPI2GPIO: [u32; 4] = [
    #[cfg(GPIO_SPI2_CS0)] GPIO_SPI2_CS0,
    #[cfg(not(GPIO_SPI2_CS0))] 0,

    #[cfg(GPIO_SPI2_CS1)] GPIO_SPI2_CS1,
    #[cfg(not(GPIO_SPI2_CS1))] 0,

    #[cfg(GPIO_SPI2_CS2)] GPIO_SPI2_CS2,
    #[cfg(not(GPIO_SPI2_CS2))] 0,

    #[cfg(GPIO_SPI2_CS3)] GPIO_SPI2_CS3,
    #[cfg(not(GPIO_SPI2_CS3))] 0,
];

#[cfg(CONFIG_STM32F7_SPI3)]
const G_SPI3GPIO: [u32; 4] = [
    #[cfg(GPIO_SPI3_CS0)] GPIO_SPI3_CS0,
    #[cfg(not(GPIO_SPI3_CS0))] 0,

    #[cfg(GPIO_SPI3_CS1)] GPIO_SPI3_CS1,
    #[cfg(not(GPIO_SPI3_CS1))] 0,

    #[cfg(GPIO_SPI3_CS2)] GPIO_SPI3_CS2,
    #[cfg(not(GPIO_SPI3_CS2))] 0,

    #[cfg(GPIO_SPI3_CS3)] GPIO_SPI3_CS3,
    #[cfg(not(GPIO_SPI3_CS3))] 0,
];

#[cfg(all(CONFIG_STM32F7_SPI1, CONFIG_NUCLEO_SPI_TEST))]
static mut SPI1: Option<ptr::NonNull<spi_dev_s>> = None;

#[cfg(all(CONFIG_STM32F7_SPI2, CONFIG_NUCLEO_SPI_TEST))]
static mut SPI2: Option<ptr::NonNull<spi_dev_s>> = None;

#[cfg(all(CONFIG_STM32F7_SPI3, CONFIG_NUCLEO_SPI_TEST))]
static mut SPI3: Option<ptr::NonNull<spi_dev_s>> = None;

/****************************************************************************
 * Public Functions
****************************************************************************/

/****************************************************************************
 * Name: stm32_spidev_initialize
*
* Description:
*   Called to configure SPI chip select GPIO pins for the Nucleo-144 board.
*
****************************************************************************/
pub fn stm32_spidev_initialize() {
    /* Configure SPI CS GPIO for output */
    #[cfg(CONFIG_STM32F7_SPI1)]
    G_SPI1GPIO
        .iter()
        .filter(|p| **p != 0)
        .for_each(|p| unsafe { stm32_configgpio(*p); });


    #[cfg(CONFIG_STM32F7_SPI2)]
    G_SPI2GPIO
        .iter()
        .filter(|p| **p != 0)
        .for_each(|p| unsafe { stm32_configgpio(*p); });


    #[cfg(CONFIG_STM32F7_SPI3)]
    G_SPI3GPIO
        .iter()
        .filter(|p| **p != 0)
        .for_each(|p| unsafe { stm32_configgpio(*p); });
}

/****************************************************************************
 * Name:  stm32_spi1/2/3/4/5/6select and stm32_spi1/2/3/4/5/6status
*
* Description:
*   The external functions, stm32_spi1/2/3/4/5/6select and
*   stm32_spi1/2/3/4/5/6status must be provided by board-specific logic.
*   They are implementations of the select and status methods of the SPI
*   interface defined by struct spi_ops_s (see include/nuttx/spi/spi.h).
*   All other methods (including stm32_spibus_initialize())
*   are provided by common STM32 logic.  To use this common SPI logic on
*   your board:
*
*   1. Provide logic in stm32_boardinitialize() to configure SPI chip select
*      pins.
*   2. Provide stm32_spi1/2/3/4/5/6select() and stm32_spi1/2/3/4/5/6status()
*      functions in your board-specific logic.  These functions will perform
*      chip selection and status operations using GPIOs in the way your
*      board is configured.
*   3. Add a calls to stm32_spibus_initialize() in your low level
*      application initialization logic
*   4. The handle returned by stm32_spibus_initialize() may then be used to
*      bind the SPI driver to higher level logic (e.g., calling
*      mmcsd_spislotinitialize(), for example, will bind the SPI driver to
*      the SPI MMC/SD driver).
*
****************************************************************************/

#[cfg(CONFIG_STM32F7_SPI1)]
#[no_mangle]
pub extern "C" fn stm32_spi1select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    let index: u32 = unsafe{ SPIDEVID_INDEX(devid) };
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    };
    
    if (G_SPI1GPIO[index] != 0) {
        unsafe {
            stm32_gpiowrite(G_SPI1GPIO[index], !selected);
        }
    }
}

#[cfg(CONFIG_STM32F7_SPI1)]
#[no_mangle]
pub extern "C" fn stm32_spi1status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}


#[cfg(CONFIG_STM32F7_SPI2)]
#[no_mangle]
pub extern "C" fn stm32_spi2select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    let index: u32 = unsafe{ SPIDEVID_INDEX(devid) };
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    };
    
    if (G_SPI2GPIO[index] != 0) {
        unsafe {
            stm32_gpiowrite(G_SPI2GPIO[index], !selected);
        }
    }
}

#[cfg(CONFIG_STM32F7_SPI2)]
#[no_mangle]
pub extern "C" fn stm32_spi2status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}


#[cfg(CONFIG_STM32F7_SPI3)]
#[no_mangle]
pub extern "C" fn stm32_spi3select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    let index: u32 = unsafe{ SPIDEVID_INDEX(devid) };
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    };
    
    if (G_SPI3GPIO[index] != 0) {
        unsafe {
            stm32_gpiowrite(G_SPI3GPIO[index], !selected);
        }
    }
}

#[cfg(CONFIG_STM32F7_SPI3)]
#[no_mangle]
pub extern "C" fn stm32_spi3status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}


#[cfg(CONFIG_STM32F7_SPI4)]
#[no_mangle]
pub extern "C" fn stm32_spi4select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    }
}

#[cfg(CONFIG_STM32F7_SPI4)]
#[no_mangle]
pub extern "C" fn stm32_spi4status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}


#[cfg(CONFIG_STM32F7_SPI5)]
#[no_mangle]
pub extern "C" fn stm32_spi5select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    }
}

#[cfg(CONFIG_STM32F7_SPI5)]
#[no_mangle]
pub extern "C" fn stm32_spi5status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}


#[cfg(CONFIG_STM32F7_SPI6)]
#[no_mangle]
pub extern "C" fn stm32_spi6select( dev: *mut spi_dev_s, devid: u32, selected: bool ) {
    unsafe {
        spiinfo("devid: %d CS: %s\n", devid as i32, if selected { "assert" } else { "de-assert" })
    }
}

#[cfg(CONFIG_STM32F7_SPI6)]
#[no_mangle]
pub extern "C" fn stm32_spi6status(_dev: *mut spi_dev_s, _devid: u32 ) -> cty::uint8_t {
    0
}

/****************************************************************************
 * Name: stm32_spi1/2/3/4/5/6cmddata
*
* Description:
*   Set or clear the SH1101A A0 or SD1306 D/C n bit to select data (true)
*   or command (false). This function must be provided by platform-specific
*   logic. This is an implementation of the cmddata method of the SPI
*   interface defined by struct spi_ops_s (see include/nuttx/spi/spi.h).
*
* Input Parameters:
*
*   spi - SPI device that controls the bus the device that requires the CMD/
*         DATA selection.
*   devid - If there are multiple devices on the bus, this selects which one
*         to select cmd or data.  NOTE:  This design restricts, for example,
*         one one SPI display per SPI bus.
*   cmd - true: select command; false: select data
*
* Returned Value:
*   None
*
****************************************************************************/

#[cfg(all(CONFIG_STM32F7_SPI1, CONFIG_SPI_CMDDATA))]
fn stm32_spi1cmddata(dev: *mut spi_dev_s , devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

#[cfg(all(CONFIG_STM32F7_SPI2, CONFIG_SPI_CMDDATA))]
fn stm32_spi2cmddata(dev: *mut spi_dev_s, devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

#[cfg(all(CONFIG_STM32F7_SPI3, CONFIG_SPI_CMDDATA))]
fn stm32_spi3cmddata(dev: *mut spi_dev_s, devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

#[cfg(all(CONFIG_STM32F7_SPI4, CONFIG_SPI_CMDDATA))]
fn stm32_spi4cmddata(dev: *mut spi_dev_s, devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

#[cfg(all(CONFIG_STM32F7_SPI5, CONFIG_SPI_CMDDATA))]
fn stm32_spi5cmddata(dev: *mut spi_dev_s, devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

#[cfg(all(CONFIG_STM32F7_SPI6, CONFIG_SPI_CMDDATA))]
fn stm32_spi6cmddata(dev: *mut spi_dev_s, devid: u32, cmd: bool) -> i32 {
    return -(ENODEV as i32);
}

// #[cfg(CONFIG_NUCLEO_SPI_TEST)]
macro_rules! spi_exchange {
    ($d:expr, $t:expr, $r:expr, $l:expr) => {
        match (*($d.as_ref().ops)).exchange {
            None => (),
            Some(func) => func($d.as_ptr(), $t, $r, $l)
        }
    };
}
// #[cfg(CONFIG_NUCLEO_SPI_TEST)]
macro_rules! spi_setmode {
    ($d:expr, $m:expr) => {
        match (*($d.as_ref().ops)).setmode {
            None => (),
            Some(func) => func($d.as_ptr(), $m)
        }
    };
}
// #[cfg(CONFIG_NUCLEO_SPI_TEST)]
macro_rules! spi_setbits {
    ($d:expr, $b:expr) => {
        match (*($d.as_ref().ops)).setbits {
            None => (),
            Some(func) => func($d.as_ptr(), $b)
        }
    };
}
// #[cfg(CONFIG_NUCLEO_SPI_TEST)]
macro_rules! spi_setfrequency {
    ($d:expr, $f:expr) => {
        match (*($d.as_ref().ops)).setfrequency {
            None => ENODEV as u32,
            Some(func) => func($d.as_ptr(), $f)
        }
    };
}

// #[cfg(CONFIG_NUCLEO_SPI_TEST)]
fn stm32_spidev_bus_test() -> i32 {

    /* Configure and test SPI- */
    let tx: *mut u8 = unsafe{
        CONFIG_NUCLEO_SPI_TEST_MESSAGE.as_mut_ptr()
    };

    // #[cfg(CONFIG_NUCLEO_SPI1_TEST)]
    unsafe {
        let spi1 = ptr::NonNull::new(stm32_spibus_initialize(1));
        match spi1 {
            None => {
                err!("ERROR Failed to initialize SPI port 1\n");
                return -(ENODEV as i32);
            }
            Some(mut spi) => {
                spi_setfrequency!(spi, CONFIG_NUCLEO_SPI1_TEST_FREQ);
                spi_setbits!(spi, CONFIG_NUCLEO_SPI1_TEST_BITS);
                spi_setmode!(spi, CONFIG_NUCLEO_SPI1_TEST_MODE);
                spi_exchange!(spi, tx, ptr::null_mut(), CONFIG_NUCLEO_SPI_TEST_MESSAGE.len());
            }
        }
    }

    #[cfg(CONFIG_NUCLEO_SPI2_TEST)]
    unsafe {
        SPI2 = ptr::NonNull::new(stm32_spibus_initialize(2));
        match SPI2 {
            None => {
                syslog(LOG_ERR, "ERROR Failed to initialize SPI port 2\n");
                return -(ENODEV as i32);
            }
            Some(mut spi) => {
                spi_setfrequency!(spi, CONFIG_NUCLEO_SPI1_TEST_FREQ);
                spi_setbits!(spi, CONFIG_NUCLEO_SPI1_TEST_BITS);
                spi_setmode!(spi, CONFIG_NUCLEO_SPI1_TEST_MODE);
                spi_exchange!(spi, tx, ptr::null_mut(), CONFIG_NUCLEO_SPI_TEST_MESSAGE.len());
            }
        }
    }

    #[cfg(CONFIG_NUCLEO_SPI3_TEST)]
    unsafe {
        SPI3 = ptr::NonNull::new(stm32_spibus_initialize(3));
        match SPI3 {
            None => {
                syslog(LOG_ERR, "ERROR Failed to initialize SPI port 3\n");
                return -(ENODEV as i32);
            }
            Some(mut spi) => {
                spi_setfrequency!(spi, CONFIG_NUCLEO_SPI1_TEST_FREQ);
                spi_setbits!(spi, CONFIG_NUCLEO_SPI1_TEST_BITS);
                spi_setmode!(spi, CONFIG_NUCLEO_SPI1_TEST_MODE);
                spi_exchange!(spi, tx, ptr::null_mut(), CONFIG_NUCLEO_SPI_TEST_MESSAGE.len());
            }
        }
    }
    OK
}   // stm32_spidev_bus_test
