/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_bringup.c
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
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_bringup
 *
 * Description:
 *   Perform architecture-specific initialization
 *
 *   CONFIG_BOARD_LATE_INITIALIZE=y :
 *     Called from board_late_initialize().
 *
 *   CONFIG_BOARD_LATE_INITIALIZE=n && CONFIG_BOARDCTL=y :
 *     Called from the NSH library
 *
 ****************************************************************************/

 //c signatures for the functions used
 extern "C" {
    fn stm32_gpiowrite(pin: u32, state: bool);
    fn nx_mount();
    fn syslog();
    fn stm32_romfs_initialize();
    fn stm32_gpio_initialize();
    fn userled_lower_initialize();
    fn stm32_adc_setup();
    fn stm32_bbsram_int();
    fn stm32_dma_alloc_init();
    fn stm32_spidev_bus_test();
    fn stm32_sdio_initialize();
    fn ferr();
    fn stm32_pwm_setup();
    fn snprintf();
    fn stm32_qencoder_initialize();
    fn stm32_can_setup();
    fn stm32_cansock_setup();
    fn stm32_i2cbus_initialize();
    fn i2c_register();
    fn kmm_zalloc();
    fn sizeof();
    fn mpu60x0_register();
    fn UNUSED();
  }

  //main function
  //returns an int
  #[no_mangle]
  pub extern "C" fn stm32_bringup() -> cty::c_int
  {
    //define int ret
    let mut ret: i32 = 0;

    //if CONFIG_I2C
    #[cfg(CONFIG_I2C)]
    {
        i2c_bus: i32;
        //pub struct i2c_master_s *i2c;
        /*struct I2CMaster {
            i2c: *mut i2c_master_s,
        }*/

        #[cfg(CONFIG_MPU60X0_I2C)]
        {
            //pub mpu_config_s *mpu_config
            /*struct I2CMaster {
                mpu_config: *mpu_config_s,
            }*/
        }
    }

    #[cfg(CONFIG_FS_PROCFS)]
    {
        /* Mount the procfs file system */
        ret = nx_mount();
        if ret < 0
        {
            syslog();
        }
    }

    #[cfg(CONFIG_STM32_ROMFS)]
    {
        /* Mount the romfs partition */

        ret = stm32_romfs_initialize();

        if ret < 0
        {
            syslog();
        }
    }

    #[cfg(CONFIG_DEV_GPIO)]
    {
        /* Register the GPIO driver */
        ret = stm32_gpio_initialize();
        if ret < 0
        {
            syslog();
            return ret;
        }
    }
    //TO-DO: CHECK TO SEE IF THIS IS CORRECT SYNTAX!!!
    #[cfg(not(all(CONFIG_ARCH_LEDS)) && (CONFIG_USERLED_LOWER))]  //if cfg!(not(CONFIG_ARCH_LEDS)) && cfg!(CONFIG_USERLED_LOWER)
    {
        ret = userled_lower_initialize();
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: userled_lower_initialize() failed: {}\n", ret);
        }
    }

    #[cfg(CONFIG_ADC)]
    {
        ret = stm32_adc_setup();
        if ret < 0
        {
            syslog();
        }
    }

    #[cfg(CONFIG_STM32F7_BBSRAM)]
    {
        pub stm32_bbsram_int();
    }

    #[cfg(CONFIG_FAT_DMAMEMORY)]
    {
        //TO-DO: may need to use let and make variable and then compare
        if stm32_dma_alloc_init() < 0
        {
            syslog();
        }
    }

    #[cfg(CONFIG_NUCLEO_SPI_TEST)]
    {
        ret = stm32_spidev_bus_test();
        //there is if ret != OK
        if 
        {
            syslog();
            return ret;
        }
    }

    #[cfg(CONFIG_MMCSD)]
    {
        ret = stm32_sdio_initialize();
        if
        {
            ferr();
            return ret;
        }
    }

    #[cfg(CONFIG_PWM)]
    {
        ret = stm32_pwm_setup();
        if ret < 0
        {
            syslog();
        }
    }

    #[cfg()]

  }