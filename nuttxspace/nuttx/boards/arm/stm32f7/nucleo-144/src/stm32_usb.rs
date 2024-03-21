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
use crate::include::*;


/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/

cfg_if::cfg_if! { // https://docs.rs/cfg-if/latest/cfg_if/
  if #[cfg(CONFIG_STM32F7_OTGFS)]
  {
    if ( cfg!(CONFIG_USBDEV) || cfg!(CONFIG_USBHOST) )
    {
      //https://stackoverflow.com/questions/45163024/whats-the-equivalent-of-a-c-preprocessor-like-define-for-an-array-length
      const HAVE_USB: i32 = 1;
    }
    else
    {
      warn!("CONFIG_STM32_OTGFS is enabled but neither CONFIG_USBDEV nor CONFIG_USBHOST");
       // #  undef HAVE_USB <- this will be dropped automatically when it goes out of scope
    }

    if( cfg!(CONFIG_NUCLEO144_USBHOST_PRIO == false )) {
      static CONFIG_NUCLEO144_USBHOST_PRIO: i32 = 100;
      // using static rather than const to avoid them being dropped when they leave scope
      // this is likely unneeded
    }

  if( cfg!(CONFIG_NUCLEO_USBHOST_STACKSIZE == false)) {
    static CONFIG_NUCLEO_USBHOST_STACKSIZE: i32 = 1024;
      // using static rather than const to avoid them being dropped when they leave scope
      // this is likely unneeded
  }

  /****************************************************************************
    * Private Data
    ****************************************************************************/

  if( cfg!(CONFIG_USBHOST) ){
    static struct usbhost_connection_s *g_usbconn;
  }

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

  if #[cfg(CONFIG_USBHOST)]{
    extern "C" fn usbhost_waiter( argc: i32, argv: &[String]) -> i32
    {
      struct usbhost_hubport_s *hport;

    println!("Running\n");
    loop
      {
        /* Wait for the device to change state */
        unsafe{
          let hport = conn_wait(&g_usbconn).expect("Failed to wait for connection state change");
        }
        if(hport.connected == true)
        {
          println!("connected\n");
        }
        else
        {
          println!("disconnected\n");
        }
        /* Did we just become connected? */

        if (hport->connected)
          {
            /* Yes.. enumerate the newly connected device */
            unsafe{
              CONN_ENUMERATE(g_usbconn, hport);
            }
          }
        }

    /* Keep the compiler from complaining */

    return 0;
  }
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
 pub extern "C" fn stm32_usbinitialize()
  {
    // pub to allow boot to use this function
    /* The OTG FS has an internal soft pull-up.
      * No GPIO configuration is required
      */

    /* Configure the OTG FS VBUS sensing GPIO,
      * Power On, and Overcurrent GPIOs
      */

    if ( cfg! (CONFIG_STM32F7_OTGFS) )
    {
      unsafe{
        stm32_configgpio(GPIO_OTGFS_VBUS);
        stm32_configgpio(GPIO_OTGFS_PWRON);
        stm32_configgpio(GPIO_OTGFS_OVER);
      }
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


     if( #[cfg(CONFIG_USBHOST)])
     {

       pub extern "C" fn stm32_usbhost_initialize()
       {
        let ret: i32;

        /* First, register all of the class drivers needed to support the drivers
          * that we care about:
          */

        println!("Register class drivers\n");
        if #[cfg(CONFIG_USBHOST_HUB)]
        {

        /* Initialize USB hub class support */
        unsafe{
          ret = usbhost_hub_initialize();
        }
        if (ret < 0)
        {
          #[error("ERROR: usbhost_hub_initialize failed: {}\n", ret)];
          return;
          }
        } /* CONFIG_USBHOST_HUB */

      if(#[cfg(CONFIG_USBHOST_MSC)])
      {
        /* Register the USB mass storage class class */
        unsafe{
          ret = usbhost_msc_initialize();
        }
        if (ret != OK)
        {
          #[error("ERROR: Failed to register the mass storage class: {}\n", ret)];
          return;
        }
      } /* CONFIG_USBHOST_MSC */

      if( #[cfg(CONFIG_USBHOST_CDCACM)] )
      {

      /* Register the CDC/ACM serial class */
      unsafe{
        ret = usbhost_cdcacm_initialize();
      }
      if (ret != OK)
        {
          #[error("ERROR: Failed to register the CDC/ACM serial class: {}\n", ret)];
        }
    } /* CONFIG_USBHOST_CDCACM */

      if(#[cfg(CONFIG_USBHOST_HIDKBD)])
      {

      /* Initialize the HID keyboard class */
      unsafe{
        ret = usbhost_kbdinit();
      }
      if (ret != OK)
        {
          #[error("ERROR: Failed to register the HID keyboard class\n")];
        }
      }/*CONFIG_USBHOST_HIDKBD */

      if(#[cfg(CONFIG_USBHOST_HIDMOUSE)])
      {

      /* Initialize the HID mouse class */
      unsafe{
        ret = usbhost_mouse_init();
      }
      if (ret != OK)
      {
          #[error("ERROR: Failed to register the HID mouse class\n")];
        }
      } /*CONFIG_USBHOST_HIDMOUSE */

    /* Then get an instance of the USB host interface */

    println!("Initialize USB host\n");
    unsafe{
      g_usbconn = stm32_otgfshost_initialize(0);
    }
    if (g_usbconn)
      {
        /* Start a thread to handle device connection. */
        println!("Start usbhost_waiter\n");
        unsafe{
          let usbhost_thread = kthread_create("usbhost", CONFIG_NUCLEO144_USBHOST_PRIO,
          CONFIG_NUCLEO_USBHOST_STACKSIZE,
          usbhost_waiter, NULL);
        }
        // CONFIG_NUCLEO144_USBHOST_PRIO ->  is defined on line 48 of this file, this is defined to be static to prevent it from dropping out of scope
        // CONFIG_NUCLEO_USBHOST_STACKSIZE -> is defined on line 52 of this file,this is defined to be static to prevent it from dropping out of scope
        // Converted using https://github.com/torvalds/linux/blob/master/include/linux/kthread.h#L15

        Ok(()) // we where successful in creating the thread if this is reached
      }
      return Err(-libc::ENODEV); // throw ENODEV which is a standard c error for invalid enviourment ( i think? )
    }
  } /* CONFIG_USBHOST */

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

    if( #[cfg(CONFIG_USBHOST)])
    {
      pub extern "C" fn stm32_usbhost_vbusdrive( iface :i32, enable: bool)
      {
         debug_assert_eq!(iface,0);
         /* Set the Power Switch by driving the active low enable pin */
         unsafe{
           stm32_gpiowrite(GPIO_OTGFS_PWRON, !enable);
         }
       }
} /* CONFIG_USBHOST */

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

      if( #[cfg(CONFIG_USBHOST)] )
      {
        // reading line 1414 of binding.rs
        pub extern "C" fn stm32_setup_overcurrent( handler : c_int, arg: *mut cty::c_void) -> i32
        {
          unsafe{
            return stm32_gpiosetevent(GPIO_OTGFS_OVER, true, true, true, handler, arg);
          }
        }
      } /* CONFIG_USBHOST */

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
      if( #[cfg(CONFIG_USBDEV)])
      {
         pub extern "C" fn stm32_usbsuspend(struct usbdev_s *dev, resume : bool)
         {
           println!("resume: {}\n", resume);
         }
      } /* CONFIG_USBDEV */
}/* CONFIG_STM32F7_OTGFS */
else{
#[no_mangle]
pub extern "C" fn stm32_usbinitialize()
{

  // pub to allow boot to use this function
  /* The OTG FS has an internal soft pull-up.
    * No GPIO configuration is required
    */

  /* Configure the OTG FS VBUS sensing GPIO,
    * Power On, and Overcurrent GPIOs
    */
  if cfg! (CONFIG_STM32F7_OTGFS)
  {
    unsafe{
      stm32_configgpio(0);
      stm32_configgpio(0);
      stm32_configgpio(0);
    }
  }

}

}
}
