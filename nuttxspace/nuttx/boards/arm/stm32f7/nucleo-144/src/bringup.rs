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
 /*extern "C" {
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
  }*/

  //NULL in rust
  /*enum Option<T> {
    None,
    Some(T),
  }*/

  //main function
  //returns an int
  #[no_mangle]
  pub extern "C" fn stm32_bringup() -> cty::c_int
  {
    //define int ret
    //define as mutable
    let mut ret: i32 = 0;

    //if CONFIG_I2C
    #[cfg(CONFIG_I2C)]
    {
        let mut i2c_bus: i32;
        //pub struct i2c_master_s *i2c;
        //should I use Box? -- its a smart pointer
        let mut i2c = &mut i2c_master_s;
        /*struct I2CMaster {
            i2c: *mut i2c_master_s,
        }*/

        #[cfg(CONFIG_MPU60X0_I2C)]
        {
            let mut mpu_config = &mpu_config_s;
            //pub mpu_config_s *mpu_config
            /*struct I2CMaster {
                mpu_config: *mpu_config_s,
            }*/
        }
    }

    #[cfg(CONFIG_FS_PROCFS)]
    {
        /* Mount the procfs file system */
        ret = nx_mount(None, STM32_PROCFS_MOUNTPOINT, "procfs", 0, None);
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: Failed to mount procfs at %s: %d\n",
            STM32_PROCFS_MOUNTPOINT, ret);
        }
    }

    #[cfg(CONFIG_STM32_ROMFS)]
    {
        /* Mount the romfs partition */

        ret = stm32_romfs_initialize();

        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: Failed to mount romfs at %s: %d\n",
            CONFIG_STM32_ROMFS_MOUNTPOINT, ret);
        }
    }

    #[cfg(CONFIG_DEV_GPIO)]
    {
        /* Register the GPIO driver */
        ret = stm32_gpio_initialize();
        if ret < 0
        {
            syslog(LOG_ERR, "Failed to initialize GPIO Driver: %d\n", ret);
            return ret;
        }
    }
    //TO-DO: CHECK TO SEE IF THIS IS CORRECT SYNTAX!!!
    #[cfg(not(all(CONFIG_ARCH_LEDS)) && (CONFIG_USERLED_LOWER))]  //if cfg!(not(CONFIG_ARCH_LEDS)) && cfg!(CONFIG_USERLED_LOWER)
    {
        ret = userled_lower_initialize(LED_DRIVER_PATH);
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
            syslog(LOG_ERR, "ERROR: stm32_adc_setup failed: %d\n", ret);
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
            syslog(LOG_ERR, "DMA alloc FAILED");
        }
    }

    #[cfg(CONFIG_NUCLEO_SPI_TEST)]
    {
        ret = stm32_spidev_bus_test();
        //there is if ret != OK
        if ret != OK
        {
            syslog(LOG_ERR, "ERROR: Failed to initialize SPI interfaces: %d\n",
            ret);
            return ret;
        }
    }

    #[cfg(CONFIG_MMCSD)]
    {
        ret = stm32_sdio_initialize();
        if ret != OK
        {
            ferr("ERROR: Failed to initialize MMC/SD driver: %d\n", ret);
            return ret;
        }
    }

    #[cfg(CONFIG_PWM)]
    {
        ret = stm32_pwm_setup();
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: stm32_pwm_setup() failed: %d\n", ret);
        }
    }

    #[cfg(CONFIG_SENSORS_QENCODER)]
        //defines an array of size 9 and initializes it to 0
        let mut buf = [i32; 9] = [0; 9];

    #[cfg(CONFIG_STM32F7_TIM1_QE)]
    {
        unsafe
        {
            snprintf(buf, buf.len(), "/dev/qe0");
        }
        ret = stm32_qencoder_initialize(buf, 1);
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: Failed to register the qencoder: %d\n",
            ret);
         return ret;
        }
    }

    #[cfg(CONFIG_STM32F7_TIM3_QE)]
    {
        unsafe
        {
            snprintf(buf, buf.len(), "/dev/qe2");
        }
        ret = stm32_qencoder_initialize(buf, 3);
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: Failed to register the qencoder: %d\n",
            ret);
         return ret;
        }
    }

    #[cfg(CONFIG_STM32F7_TIM4_QE)]
    {
        unsafe
        {
            snprintf(buf, buf.len(), "/dev/qe3");
        }
        ret = stm32_qencoder_initialize(buf, 4);
        if ret < 0
        {
            syslog(LOG_ERR, "ERROR: Failed to register the qencoder: %d\n",
            ret);
         return ret;
        }
    }

    #[cfg(CONFIG_STM32F7_CAN_CHARDRIVER)]
    {
        ret = stm32_can_setup();
        if ret < 0
          {
            syslog(LOG_ERR, "ERROR: stm32f7_can_setup failed: %d\n", ret);
            return ret;
          }
    }

    #[cfg(CONFIG_STM32F7_CAN_SOCKET)]
    {
        ret = stm32_cansock_setup();
        if ret < 0
          {
            syslog(LOG_ERR, "ERROR: stm32_cansock_setup failed: %d\n", ret);
          }
    }

    //TO-DO: Check if correct
    #[cfg(all(CONFIG_I2C)) && (CONFIG_STM32F7_I2C1)]
    {
        i2c_bus = 1;
        i2c = stm32_i2cbus_initialize(i2c_bus);

        if i2c == None
        {
            syslog(LOG_ERR, "ERROR: Failed to get I2C%d interface\n", i2c_bus);
        }
        else
        {
            #[cfg(CONFIG_SYSTEM_I2CTOOL)]
            {
                ret = i2c_register(i2c, i2c_bus);
            
                if ret < 0
                {
                    syslog(LOG_ERR, "ERROR: Failed to register I2C%d driver: %d\n", i2c_bus, ret);
                }
            }

            #[cfg(CONFIG_MPU60X0_I2C)]
            {
                mpu_config = kmm_zalloc(sizeof(struct mpu_config_s));
                if (mpu_config == NULL)
                {
                  syslog(LOG_ERR, "ERROR: Failed to allocate mpu60x0 driver\n");
                }
                else
                {
                  mpu_config->i2c = i2c;
                  mpu_config->addr = 0x68;
                  mpu60x0_register("/dev/imu0", mpu_config);
                }
            }
        }
    }

    UNUSED(ret);
    return OK;

  }