/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_usb.c
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
#![cfg(CONFIG_STM32F7_OTGFS)]

use crate::bindings::*;
use core::ptr;
use crate::{err, info};

/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/

#[cfg(not(any(CONFIG_USBDEV, CONFIG_USBHOST)))]
compile_error!("CONFIG_STM32_OTGFS is enabled but neither CONFIG_USBDEV nor CONFIG_USBHOST");

#[cfg(not(CONFIG_NUCLEO144_USBHOST_PRIO))]
const CONFIG_NUCLEO144_USBHOST_PRIO: i32 = 100;

#[cfg(not(CONFIG_NUCLEO_USBHOST_STACKSIZE))]
const CONFIG_NUCLEO_USBHOST_STACKSIZE: i32 = 1024;

/****************************************************************************
 * Private Data
 ****************************************************************************/

#[cfg(CONFIG_USBHOST)]
static mut G_USBCONN: Option<ptr::NonNull<usbhost_connection_s>> = None;

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Name: usbhost_waiter
 *
 * Description:
 *   Wait for USB devices to be connected.
 *
 ****************************************************************************/

#[cfg(CONFIG_USBHOST)]
#[no_mangle]
unsafe extern "C" fn usbhost_waiter(argc: i32, argv: *mut *mut u8) -> i32 {
    use core::mem::MaybeUninit;
    let mut hport: MaybeUninit<*mut usbhost_hubport_s> = MaybeUninit::uninit();

    loop {
        if let Some(mut conn) = G_USBCONN {
            conn.as_ref().wait.unwrap()(conn.as_ptr(), hport.assume_init_mut() as *mut _);

            if !hport.assume_init().is_null() && (*hport.assume_init()).connected {
                conn.as_ref().enumerate.unwrap()(conn.as_ptr(), hport.assume_init());
            }
        }
    }
    OK
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_usbinitialize
 *
 * Description:
 *   Called from stm32_usbinitialize very early in inialization to setup
 *   USB-related GPIO pins for the nucleo-144 board.
 *
 ****************************************************************************/
#[no_mangle]
pub extern "C" fn stm32_usbinitialize() {
    /* The OTG FS has an internal soft pull-up.
     * No GPIO configuration is required
     */

    /* Configure the OTG FS VBUS sensing GPIO,
     * Power On, and Overcurrent GPIOs
     */

    #[cfg(CONFIG_STM32F7_OTGFS)]
    unsafe {
        stm32_configgpio(GPIO_OTGFS_VBUS.into());
        stm32_configgpio(GPIO_OTGFS_PWRON.into());
        stm32_configgpio(GPIO_OTGFS_OVER.into());
    }
}

/****************************************************************************
 * Name: stm32_usbhost_initialize
 *
 * Description:
 *   Called at application startup time to initialize the USB host
 *   functionality.
 *   This function will start a thread that will monitor for device
 *   connection/disconnection events.
 *
 ****************************************************************************/

#[cfg(CONFIG_USBHOST)]
pub extern "C" fn stm32_usbhost_initialize() -> i32 {
    info!("Register class drivers\n");

    #[cfg(CONFIG_USBHOST_HUB)]
    match unsafe { usbhost_hub_initialize() } {
        OK => (),
        ret => err!("ERROR: usbhost_hub_initialize failed: %d\n\0", ret)
    }

    #[cfg(CONFIG_USBHOST_MSC)]
    match unsafe { usbhost_msc_initialize() } {
        OK => (),
        ret => err!("ERROR: Failed to register the mass storage class: %d\n\0", ret)
    }

    #[cfg(CONFIG_USBHOST_CDCACM)]
    match unsafe { usbhost_cdcacm_initialize() } {
        OK => (),
        ret => err!("ERROR: Failed to register the CDC/ACM serial class: %d\n\0", ret)
    }

    #[cfg(CONFIG_USBHOST_HIDKBD)]
    match unsafe { usbhost_kbdinit() } {
        OK => (),
        ret => err!("ERROR: Failed to register the HID keyboard class\n\0", ret)
    }

    #[cfg(CONFIG_USBHOST_HIDMOUSE)]
    match unsafe { usbhost_mouse_init() } {
        OK => (),
        ret => err!("ERROR: Failed to register the HID mouse class\n\0", ret)
    }

    info!("Initialize USB host\n");

    /* Then get an instance of the USB host interface */
    unsafe {
        G_USBCONN = ptr::NonNull::new(stm32_otgfshost_initialize(0));
        if G_USBCONN.is_none() {
            return -(ENODEV as i32);
        }

        info!("Start usbhost_waiter\n");

        return match kthread_create(
            b"usbhost" as *const u8,
            CONFIG_NUCLEO144_USBHOST_PRIO,
            CONFIG_NUCLEO_USBHOST_STACKSIZE,
            Some(usbhost_waiter),
            ptr::null_mut(),
        ) {
            OK => OK,
            _ => -(ENODEV as i32),
        };
    }
}

/****************************************************************************
 * Name: stm32_usbhost_vbusdrive
 *
 * Description:
 *   Enable/disable driving of VBUS 5V output.  This function must be
 *   provided be each platform that implements the STM32 OTG FS host
 *   interface
 *
 *   "On-chip 5 V VBUS generation is not supported. For this reason, a
 *    charge pump or, if 5 V are available on the application board, a
 *    basic power switch, must be added externally to drive the 5 V VBUS
 *    line. The external charge pump can be driven by any GPIO output.
 *    When the application decides to power on VBUS using the chosen GPIO,
 *    it must also set the port power bit in the host port control and
 *    status register (PPWR bit in OTG_FS_HPRT).
 *
 *   "The application uses this field to control power to this port,
 *    and the core clears this bit on an overcurrent condition."
 *
 * Input Parameters:
 *   iface - For future growth to handle multiple USB host interface.
 *           Should be zero.
 *   enable - true: enable VBUS power; false: disable VBUS power
 *
 * Returned Value:
 *   None
 *
 ****************************************************************************/

#[cfg(CONFIG_USBHOST)]
#[no_mangle]
pub extern "C" fn stm32_usbhost_vbusdrive(iface: i32, enable: bool) {
    debug_assert!(iface == 0);

    unsafe{
        stm32_gpiowrite(GPIO_OTGFS_PWRON, !enable);
    }
}

/****************************************************************************
 * Name: stm32_setup_overcurrent
 *
 * Description:
 *   Setup to receive an interrupt-level callback if an overcurrent
 *   condition is detected.
 *
 * Input Parameters:
 *   handler - New overcurrent interrupt handler
 *   arg     - The argument provided for the interrupt handler
 *aa
 * Returned Value:
 *   Zero (OK) is returned on success.  Otherwise, a negated errno value
 *   is returned to indicate the nature of the failure.
 *
 ****************************************************************************/

#[cfg(CONFIG_USBHOST)]
#[no_mangle]
pub unsafe extern "C" fn stm32_setup_overcurrent(handler: xcpt_t, arg: *mut cty::c_void) -> i32 {
    stm32_gpiosetevent(GPIO_OTGFS_OVER as u32, true, true, true, handler, arg)
}

/****************************************************************************
 * Name:  stm32_usbsuspend
 *
 * Description:
 *   Board logic must provide the stm32_usbsuspend logic if the USBDEV
 *   driver is used. This function is called whenever the USB enters or
 *   leaves suspend mode. This is an opportunity for the board logic to
 *   shutdown clocks, power, etc. while the USB is suspended.
 *
 ****************************************************************************/

#[cfg(CONFIG_USBDEV)]
pub extern "C" fn stm32_usbsuspend(dev: *mut usbdev_s, resume: bool) {
    info!("resume: %d\n", resume);
}