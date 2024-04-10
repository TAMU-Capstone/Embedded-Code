/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_composite.c
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
use core::ptr::null_mut;
/* Not found in this scope
g_mschandle
uerr
usbmsc_classobject
*/

/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/
const COMPOSITE0_DEV: usize = 3;

/****************************************************************************
 * Private Data
 ****************************************************************************/
 #[cfg(CONFIG_USBMSC_COMPOSITE)]
 static g_mschandle: *mut ();

 /****************************************************************************
 * Private Functions
 ****************************************************************************/
/****************************************************************************
 * Name: board_mscclassobject
 *
 * Description:
 *   If the mass storage class driver is part of composite device, then
 *   its instantiation and configuration is a multi-step, board-specific,
 *   process (See comments for usbmsc_configure below).  In this case,
 *   board-specific logic must provide board_mscclassobject().
 *
 *   board_mscclassobject() is called from the composite driver.  It must
 *   encapsulate the instantiation and configuration of the mass storage
 *   class and the return the mass storage device's class driver instance
 *   to the composite driver.
 *
 * Input Parameters:
 *   classdev - The location to return the mass storage class' device
 *     instance.
 *
 * Returned Value:
 *   0 on success; a negated errno on failure
 *
 ****************************************************************************/

 #[cfg(CONFIG_USBMSC_COMPOSITE)] 
 fn board_mscclassobject_impl(minor: i32, devinfo: &mut usbdev_devinfo_s, classdev: &mut *mut usbdevclass_driver_s) -> i32 {

    let ret: i32 = 0;
    // g_mschandle not found in this scope 
    debug_assert!(g_mschandle.is_null(), "g_mschandle is null");

    /* Configure the mass storage device */
    
    // see how josh used print outs 
    //uinfo("Configuring with NLUNS=1\n");
    ret = unsafe {
        usbmsc_configure(1, &g_mschandle)
    };

    if ret < 0 {
        /* May not really need this
        unsafe {
            // uerr will either be replaced by _err or _none during preprocessing.
            uerr(b"ERROR: usbmsc_configure failed: %d\n".as_ptr(), -ret);
        }
        */
        return ret;
    }

    //uinfo("MSC handle=%p\n", g_mschandle);

    /* Bind the LUN(s) */
    // uinfo("Bind LUN=0 to /dev/mmcsd0\n");
    ret = unsafe {
        usbmsc_bindlun(g_mschandle, "/dev/mmcsd0".as_ptr(), 0, 0, 0, false)
    };

    if ret < 0 {
        unsafe{
            //uerr("ERROR: usbmsc_bindlun failed for LUN 1 at /dev/mmcsd0: %d\n", ret);
            usbmsc_uninitialize(g_mschandle);
            g_mschandle = null_mut();
            return ret; // may need to change
        }
    }

    /* Get the mass storage device's class object */
    ret = unsafe { usbmsc_classobject(g_mschandle, devinfo, classdev) };

    if ret < 0 {
      unsafe{
        uerr("ERROR: usbmsc_classobject failed: %d\n", -ret);
        usbmsc_uninitialize(g_mschandle);
        g_mschandle = null_mut();
      }
    }
    return ret;
 }
 // end CONFIG_USBMSC_COMPOSITE


 /****************************************************************************
 * Name: board_mscuninitialize
 *
 * Description:
 *   Un-initialize the USB storage class driver.
 *   This is just an application specific wrapper for usbmsc_unitialize()
 *   that is called form the composite device logic.
 *
 * Input Parameters:
 *   classdev - The class driver instrance previously give to the composite
 *     driver by board_mscclassobject().
 *
 * Returned Value:
 *   None
 *
 *******************************************************/
#[cfg(CONFIG_USBMSC_COMPOSITE)]
unsafe fn board_mscuninitialize(classdev: *mut usbdevclass_driver_s) {
    if !g_mschandle.is_null() {
        unsafe{ usbmsc_uninitialize(g_mschandle) };
    }
    g_mschandle = null_mut();
}

/****************************************************************************
 * Name:  board_composite_connect
 *
 * Description:
 *   Connect the USB composite device on the specified USB device port for
 *   configuration 0.
 *
 * Input Parameters:
 *   port     - The USB device port.
 *
 * Returned Value:
 *   A non-NULL handle value is returned on success.  NULL is returned on
 *   any failure.
 *
 ****************************************************************************/
 //static void *board_composite0_connect(int port)
/*
 fn board_composite0_connect(port: i32) -> *mut() {
    
    let mut dev: [composite_devdesc_s; COMPOSITE0_DEV] = [composite_devdesc_s::default(); COMPOSITE0_DEV];




    
    null_mut() // temp
} // end board_composite0_connect
*/