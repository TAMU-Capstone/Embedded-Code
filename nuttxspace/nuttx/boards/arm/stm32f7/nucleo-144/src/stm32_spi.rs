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
use crate::bindings::*;
cfg_if::cfg_if! {

   if #[cfg(CONFIG_SPI)]
{
   /****************************************************************************
  * Pre-processor Definitions
  ****************************************************************************/

 if ( cfg!(CONFIG_NUCLEO_SPI1_TEST) )
 {

    if ( cfg!(CONFIG_NUCLEO_SPI1_TEST_MODE0) )
    {
          static CONFIG_NUCLEO_SPI1_TEST_MODE SPIDEV_MODE0: i32 = 1;
    }
    elif ( cfg!(CONFIG_NUCLEO_SPI1_TEST_MODE1) )
    {
          static CONFIG_NUCLEO_SPI1_TEST_MODE SPIDEV_MODE1: i32 = 1;
    }
   elif ( cfg!(CONFIG_NUCLEO_SPI1_TEST_MODE2) )
   {
     static CONFIG_NUCLEO_SPI1_TEST_MODE SPIDEV_MODE2: i32 = 1;
   }
    elif ( cfg!(CONFIG_NUCLEO_SPI1_TEST_MODE3) )
    {
      static CONFIG_NUCLEO_SPI1_TEST_MODE SPIDEV_MODE3: i32 = 1;
    }
  else{
   #[error("No CONFIG_NUCLEO_SPI1_TEST_MODEx defined")];
  }
} // CONFIG_NUCLEO_SPI1_TEST

 if ( cfg!(CONFIG_NUCLEO_SPI2_TEST) )
 {
   if ( cfg!(CONFIG_NUCLEO_SPI2_TEST_MODE0) )
   {
     static CONFIG_NUCLEO_SPI2_TEST_MODE SPIDEV_MODE0: i32 = 1;
    }
    elif ( cfg! (CONFIG_NUCLEO_SPI2_TEST_MODE1) )
    {
      static CONFIG_NUCLEO_SPI2_TEST_MODE SPIDEV_MODE1: i32 = 1;
    }
     elif ( cfg! (CONFIG_NUCLEO_SPI2_TEST_MODE2))
     {
       static CONFIG_NUCLEO_SPI2_TEST_MODE SPIDEV_MODE2: i32 = 1;
     }
    elif ( cfg! (CONFIG_NUCLEO_SPI2_TEST_MODE3) )
    {
      static CONFIG_NUCLEO_SPI2_TEST_MODE SPIDEV_MODE3: i32 = 1;
    }
   else
   {
     #[error("No CONFIG_NUCLEO_SPI2_TEST_MODEx defined")];
    }
} // CONFIG_NUCLEO_SPI2_TEST

 if ( cfg!(CONFIG_NUCLEO_SPI3_TEST) )
 {
     if ( cfg!(CONFIG_NUCLEO_SPI3_TEST_MODE0) )
     {
       static CONFIG_NUCLEO_SPI3_TEST_MODE SPIDEV_MODE0: i32 = 1;
     }
     elif ( cfg!(CONFIG_NUCLEO_SPI3_TEST_MODE1) )
     {
       static CONFIG_NUCLEO_SPI3_TEST_MODE SPIDEV_MODE1: i32 = 1;
     }
   elif ( cfg! (CONFIG_NUCLEO_SPI3_TEST_MODE2))
   {
     static CONFIG_NUCLEO_SPI3_TEST_MODE SPIDEV_MODE2: i32 = 1;
   }
   elif (cfg!(CONFIG_NUCLEO_SPI3_TEST_MODE3))
   {
     static CONFIG_NUCLEO_SPI3_TEST_MODE SPIDEV_MODE3: i32 = 1;
   }
  else
  {
   #[error("No CONFIG_NUCLEO_SPI3_TEST_MODEx defined")];
  }
} // CONFIG_NUCLEO_SPI3_TEST

 /****************************************************************************
  * Private Data
  ****************************************************************************/

 if (cfg!(CONFIG_STM32F7_SPI1))
 {
  let mut g_spi1gpio = [0; 4];
  if (cfg!(GPIO_SPI1_CS0))
  {
    g_spi1gpio[0] = GPIO_SPI1_CS0;
  }
  else
  {
    g_spi1gpio[0] = 0;
  }
  if (cfg! (GPIO_SPI1_CS1) )
  {
    g_spi1gpio[1] = GPIO_SPI1_CS1;
  }
  else
  {
    g_spi1gpio[1] = 0;
  }
 if (cfg! (GPIO_SPI1_CS2) )
 {
   g_spi1gpio[2] = GPIO_SPI1_CS2;
  }
  else
  {
    g_spi1gpio[2] = GPIO_SPI1_CS2;
  }
  if (cfg!(GPIO_SPI1_CS3))
  {

    g_spi1gpio[3] = GPIO_SPI1_CS3;
  }
  else
  {
    g_spi1gpio[3] = 0;
  }
 } // CONFIG_STM32F7_SPI1

 if (cfg!(CONFIG_STM32F7_SPI2))
 {
  let mut g_spi2gpio = [0; 4];
  if (cfg!(GPIO_SPI2_CS0))
  {
    g_spi2gpio[0] = GPIO_SPI2_CS0;
  }
  else
  {
    g_spi2gpio[0] = 0;
  }
  if (cfg! (GPIO_SPI2_CS1) )
  {
    g_spi2gpio[1] = GPIO_SPI2_CS1;
  }
  else
  {
    g_spi2gpio[1] = 0;
  }
 if (cfg! (GPIO_SPI2_CS2) )
 {
   g_spi2gpio[2] = GPIO_SPI2_CS2;
  }
  else
  {
    g_spi2gpio[2] = GPIO_SPI2_CS2;
  }
  if (cfg!(GPIO_SPI2_CS3))
  {

    g_spi2gpio[3] = GPIO_SPI2_CS3;
  }
  else
  {
    g_spi2gpio[3] = 0;
  }
 } // CONFIG_STM32F7_SPI2

 if (cfg!(CONFIG_STM32F7_SPI3))
 {
  let mut g_spi3gpio = [0; 4];
  if (cfg!(GPIO_SPI3_CS0))
  {
    g_spi3gpio[0] = GPIO_SPI3_CS0;
  }
  else
  {
    g_spi3gpio[0] = 0;
  }
  if (cfg! (GPIO_SPI3_CS1) )
  {
    g_spi3gpio[1] = GPIO_SPI3_CS1;
  }
  else
  {
    g_spi3gpio[1] = 0;
  }
 if (cfg! (GPIO_SPI3_CS2) )
 {
   g_spi3gpio[2] = GPIO_SPI3_CS2;
  }
  else
  {
    g_spi3gpio[2] = GPIO_SPI3_CS2;
  }
  if (cfg!(GPIO_SPI3_CS3))
  {

    g_spi3gpio[3] = GPIO_SPI3_CS3;
  }
  else
  {
    g_spi3gpio[3] = 0;
  }
 } // CONFIG_STM32F7_SPI3

 if (cfg!(CONFIG_NUCLEO_SPI_TEST))
 {
   if (cfg!(CONFIG_STM32F7_SPI1))
   {
          // DONT TRUST THIS STRUGGLING TO FIND A SOLID LIFT AND SHIFT HERE
     let mut spi1: *mut spi_dev_s;
    }
    if (cfg! (CONFIG_STM32F7_SPI2));
    {
            // DONT TRUST THIS STRUGGLING TO FIND A SOLID LIFT AND SHIFT HERE
      let mut spi2: *mut spi_dev_s;
    }
    if (cfg!(CONFIG_STM32F7_SPI3))
    {
            // DONT TRUST THIS STRUGGLING TO FIND A SOLID LIFT AND SHIFT HERE
      let mut spi3: *mut spi_dev_s;
    }
}// CONFIG_NUCLEO_SPI_TEST

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


// weak function used here, no direct rust equivalent, hoping to ignore but I really think this wont work

pub fn stm32_spidev_initialize()
 {
   /* Configure SPI CS GPIO for output */

 if(cfg!(CONFIG_STM32F7_SPI1)){
  for i in g_spi1gpio.iter();
   {
     if (*i != 0)
     {
      unsafe{
        stm32_configgpio(*i);
        }
      }
    }
  } //CONFIG_STM32F7_SPI1

 if(cfg!(CONFIG_STM32F7_SPI2)){
  for i in g_spi2gpio.iter();
   {
     if (*i != 0)
     {
      unsafe{
        stm32_configgpio(*i);
        }
      }
    }
  } // CONFIG_STM32F7_SPI2

  if(cfg!(CONFIG_STM32F7_SPI3)){
    for i in g_spi3gpio.iter();
     {
       if (*i != 0)
       {
        unsafe{
          stm32_configgpio(*i);
          }
        }
      }
    } // CONFIG_STM32F7_SPI3
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

  if(#[cfg!(CONFIG_STM32F7_SPI1)])
  {

    pub extern "C" fn stm32_spi1select( dev: *mut spi_dev_s, devid : uint32_t , select : bool )
      {
        unsafe{
          let index : _uint32_t = SPIDEVID_INDEX(devid);
        }
        unsafe{
          spiinfo("devid: %d CS: %s\n",
          (int)devid, selected ? "assert" : "de-assert");
        }

        if (g_spi1gpio[index] != 0)
          {
            unsafe{
              stm32_gpiowrite(g_spi1gpio[index], !selected);
            }
          }
      }

 fn stm32_spi1status(dev : *mut spi_dev_s , devid : _uint32_t ) -> __uint8_t
 {
  return 0;
 }
} // CONFIG_STM32F7_SPI1

 if(#[cfg(CONFIG_STM32F7_SPI2)])
 {

   extern "C" fn stm32_spi2select( dev : *mut spi_dev_s,
     devid : uint32_t, selected : bool)
 {
    unsafe{
      let index : uint32_t = SPIDEVID_INDEX(devid);
    }
    unsafe{
      spiinfo("devid: %d CS: %s\n",
              (int)devid, selected ? "assert" : "de-assert");
    }

   if (g_spi2gpio[index] != 0)
     {
      unsafe{
        stm32_gpiowrite(g_spi2gpio[index], !selected);
      }
     }
 }

 fn stm32_spi2status(dev : *mut spi_dev_s , devid : _uint32_t ) -> __uint8_t
 {
  return 0;
 }
} // CONFIG_STM32F7_SPI2

if(#[cfg(CONFIG_STM32F7_SPI3)])
{

  extern "C" fn stm32_spi3select( dev : *mut spi_dev_s,
    devid : uint32_t, selected : bool)
{
   unsafe{
     let index : uint32_t = SPIDEVID_INDEX(devid);
   }
   unsafe{
     spiinfo("devid: %d CS: %s\n",
             (int)devid, selected ? "assert" : "de-assert");
   }

  if (g_spi3gpio[index] != 0)
    {
     unsafe{
       stm32_gpiowrite(g_spi2gpio[index], !selected);
     }
    }
}

fn stm32_spi3status(dev : *mut spi_dev_s , devid : _uint32_t ) -> __uint8_t
{
 return 0;
}
} // CONFIG_STM32F7_SPI3

 if(#[cfg(CONFIG_STM32F7_SPI4)]){

   extern "C" fn stm32_spi4select( dev : * mut spi_dev_s,
    devid : uint32_t, selected : bool)
    {
      unsafe{
        spiinfo("devid: %d CS: %s\n",
                (int)devid, selected ? "assert" : "de-assert");
      }
 }

 fn stm32_spi4status( dev : * mut spi_dev_s,  devid : uint32_t) -> uint8_t
 {
   return 0;
  }
} // CONFIG_STM32F7_SPI4

if(#[cfg(CONFIG_STM32F7_SPI5)]){

  extern "C" fn stm32_spi5select( dev : * mut spi_dev_s,
   devid : uint32_t, selected : bool)
   {
     unsafe{
       spiinfo("devid: %d CS: %s\n",
               (int)devid, selected ? "assert" : "de-assert");
     }
}

fn stm32_spi5status( dev : * mut spi_dev_s,  devid : uint32_t) -> uint8_t
{
  return 0;
 }
} // CONFIG_STM32F7_SPI5

if(#[cfg(CONFIG_STM32F7_SPI6)]){

  extern "C" fn stm32_spi6select( dev : * mut spi_dev_s,
   devid : uint32_t, selected : bool)
   {
     unsafe{
       spiinfo("devid: %d CS: %s\n",
               (int)devid, selected ? "assert" : "de-assert");
     }
}

fn stm32_spi6status( dev : * mut spi_dev_s,  devid : uint32_t) -> uint8_t
{
  return 0;
 }
} // CONFIG_STM32F7_SPI6


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

 if #[cfg( CONFIG_SPI_CMDDATA )]
 {
   if #[cfg(CONFIG_STM32F7_SPI1)]
   {

    fn stm32_spi1cmddata(dev: *mut spi_dev_s , devid : uint32_td, cm : boold -> i32)
     {
       return -ENODEV;
      }
    }

 if #[cfg(CONFIG_STM32F7_SPI2)]
 {

   if stm32_spi2cmddata(dev: *mut spi_dev_s, devid, : uint32_t bmd) : bool
 -> i32   {
     return -ENODEV;
    }
  }

 if #[cfg(CONFIG_STM32F7_SPI3)]
 {

   if stm32_spi3cmddata(dev: *mut spi_dev_s, devid, : uint32_t bmd) : bool
 -> i32   {
     return -ENODEV;
    }
  }

  if #[cfg(CONFIG_STM32F7_SPI4)]
  {

    fn stm32_spi4cmddata(dev: *mut spi_dev_s, devid : uint32_t, cmd : bool) -> i32
    {
      return -ENODEV;
    }
  }

  if #[cfg(CONFIG_STM32F7_SPI5)]
  {

    fn stm32_spi5cmddata(dev: *mut spi_dev_s, devid : uint32_t, cmd : bool) -> i32
    {
      return -ENODEV;
    }
  }

  if #[cfg(CONFIG_STM32F7_SPI6)]
  {

    fn stm32_spi6cmddata(dev: *mut spi_dev_s, devid : uint32_t, cmd : bool) -> i32
    {
      return -ENODEV;
    }
  }

} // CONFIG_SPI_CMDDATA

 if (#cfg(CONFIG_NUCLEO_SPI_TEST))
 {

   fn stm32_spidev_bus_test() -> i32
   {
   /* Configure and test SPI- */
    unsafe{
      let tx : *mux uint8_t  = CONFIG_NUCLEO_SPI_TEST_MESSAGE as *mux uint8_t;
    }

 if #[cfg(CONFIG_NUCLEO_SPI1_TEST)]
 {
  unsafe{
    spi1 = stm32_spibus_initialize(1);
  }

   if (!spi1)
   {
    unsafe{
      syslog(LOG_ERR, "ERROR Failed to initialize SPI port 1\n");
    }
     return -ENODEV;
    }

   /* Default SPI1 to NUCLEO_SPI1_FREQ and mode */
    unsafe{
      SPI_SETFREQUENCY(spi1, CONFIG_NUCLEO_SPI1_TEST_FREQ);
      SPI_SETBITS(spi1, CONFIG_NUCLEO_SPI1_TEST_BITS);
      SPI_SETMODE(spi1, CONFIG_NUCLEO_SPI1_TEST_MODE);
      SPI_EXCHANGE(spi1, tx, NULL, nitems(CONFIG_NUCLEO_SPI_TEST_MESSAGE));
    }
} // CONFIG_NUCLEO_SPI1_TEST

 if #[cfg(CONFIG_NUCLEO_SPI2_TEST)]
 {
  unsafe{
    spi2 = stm32_spibus_initialize(2);
  }
   
   if (!spi2)
     {
      unsafe{
        syslog(LOG_ERR, "ERROR Failed to initialize SPI port 2\n");
      }
       return -ENODEV;
      }
      
      /* Default SPI2 to NUCLEO_SPI2_FREQ and mode */
      unsafe{
        SPI_SETFREQUENCY(spi2, CONFIG_NUCLEO_SPI2_TEST_FREQ);
        SPI_SETBITS(spi2, CONFIG_NUCLEO_SPI2_TEST_BITS);
        SPI_SETMODE(spi2, CONFIG_NUCLEO_SPI2_TEST_MODE);
        SPI_EXCHANGE(spi2, tx, NULL, nitems(CONFIG_NUCLEO_SPI_TEST_MESSAGE));
      }
    }
      
  if #[cfg(CONFIG_NUCLEO_SPI3_TEST)]
  {
    unsafe{
      spi3 = stm32_spibus_initialize(3);
    }
    
   if (!spi3)
     {
      unsafe{
        syslog(LOG_ERR, "ERROR Failed to initialize SPI port 2\n");
      }
       return -ENODEV;
     }

   /* Default SPI3 to NUCLEO_SPI3_FREQ and mode */
  unsafe{
    SPI_SETFREQUENCY(spi3, CONFIG_NUCLEO_SPI3_TEST_FREQ);
    SPI_SETBITS(spi3, CONFIG_NUCLEO_SPI3_TEST_BITS);
    SPI_SETMODE(spi3, CONFIG_NUCLEO_SPI3_TEST_MODE);
    SPI_EXCHANGE(spi3, tx, NULL, nitems(CONFIG_NUCLEO_SPI_TEST_MESSAGE));
  }
  }
   
   return OK;
  }
} // CONFIG_NUCLEO_SPI_TEST
} // defined(CONFIG_SPI)
else{
  // considering creating a empty config that will catch and allow this to be a fake preprocessor
  pub extern "C" fn stm32_spidev_initialize()
  {
    /* Configure SPI CS GPIO for output */
      }
}
} // if_cfg
