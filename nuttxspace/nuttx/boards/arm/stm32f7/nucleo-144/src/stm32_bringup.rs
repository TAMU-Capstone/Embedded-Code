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
use crate::bindings::*;
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

  //main function
  //returns an int
  #[no_mangle]
  pub extern "C" fn stm32_bringup() -> cty::c_int
  {
    //define int ret
    //define as mutable
    let mut ret: i32 = 0;

    //create a null ptr to replace NULL in C
    let mut null_ptr: *const u8 = 0 as *const u8;

    //if CONFIG_I2C
    if cfg!(CONFIG_I2C)
    {
        let mut i2c_bus: i32;
        //should I use Box? -- its a smart pointer
        let mut i2c = &mut i2c_master_s { ops: val };

        if cfg!(CONFIG_MPU60X0_I2C)
        {
            let mpu_config = mpu_config_s { i2c: val, addr: val };
        }
    }

    if cfg!(CONFIG_FS_PROCFS)
    {
        /* Mount the procfs file system */
        ret = nx_mount(null_ptr, STM32_PROCFS_MOUNTPOINT, "procfs", 0, null_ptr);
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to mount procfs at %s: %d\n": &str, STM32_PROCFS_MOUNTPOINT, ret: i32);
        }
    }

    if cfg!(CONFIG_STM32_ROMFS)
    {
        /* Mount the romfs partition */

        ret = stm32_romfs_initialize();

        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to mount romfs at %s: %d\n": &str, CONFIG_STM32_ROMFS_MOUNTPOINT, ret: i32);
        }
    }

    if cfg!(CONFIG_DEV_GPIO)
    {
        /* Register the GPIO driver */
        ret = stm32_gpio_initialize();
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "Failed to initialize GPIO Driver: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(not(CONFIG_ARCH_LEDS)) && cfg!(CONFIG_USERLED_LOWER)
    {
        ret = userled_lower_initialize(LED_DRIVER_PATH);
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: userled_lower_initialize() failed: {}\n": &str, ret: i32);
        }
    }

    if cfg!(CONFIG_ADC)
    {
        ret = stm32_adc_setup();
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: stm32_adc_setup failed: %d\n": &str, ret: i32);
        }
    }

    if cfg!(CONFIG_STM32F7_BBSRAM)
    {
        pub fn stm32_bbsram_int();
    }

    if cfg!(CONFIG_FAT_DMAMEMORY)
    {
        //TO-DO: may need to use let and make variable and then compare
        let temp = stm32_dma_alloc_init();
        if temp < 0
        {
            pub fn syslog(LOG_ERR: u8, "DMA alloc FAILED": &str);
        }
    }

    if cfg!(CONFIG_NUCLEO_SPI_TEST)
    {
        ret = stm32_spidev_bus_test();
        //there is if ret != OK
        if ret != OK
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to initialize SPI interfaces: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(CONFIG_MMCSD)
    {
        ret = stm32_sdio_initialize();
        if ret != OK
        {
            pub fn ferr("ERROR: Failed to initialize MMC/SD driver: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(CONFIG_PWM)
    {
        ret = stm32_pwm_setup();
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: stm32_pwm_setup() failed: %d\n": &str, ret: i32);
        }
    }

    if cfg!(CONFIG_SENSORS_QENCODER)
    {
        //defines an array of size 9 and initializes it to 0
        let mut buf: [i32; 9] = [0; 9];
    }
    
    if cfg!(CONFIG_STM32F7_TIM1_QE)
    {
        pub fn snprintf(buf: &[i32], buf.len(): i32, "/dev/qe0": &str);
        ret = stm32_qencoder_initialize(buf: &[i32], 1);
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to register the qencoder: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(CONFIG_STM32F7_TIM3_QE)
    {
        pub fn snprintf(buf: &[i32], sizeof(buf): i32, "/dev/qe2": &str);
        ret = stm32_qencoder_initialize(buf: &[i32], 3);
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to register the qencoder: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(CONFIG_STM32F7_TIM4_QE)
    {
        pub fn snprintf(buf: &[i32], buf.len(): i32, "/dev/qe3": &str);
        ret = stm32_qencoder_initialize(buf: &[i32], 4);
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to register the qencoder: %d\n": &str, ret: i32);
         return ret;
        }
    }

    if cfg!(CONFIG_STM32F7_CAN_CHARDRIVER)
    {
        ret = stm32_can_setup();
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: stm32f7_can_setup failed: %d\n": &str, ret: i32);
            return ret;
        }
    }

    if cfg!(CONFIG_STM32F7_CAN_SOCKET)
    {
        ret = stm32_cansock_setup();
        if ret < 0
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: stm32_cansock_setup failed: %d\n": &str, ret: i32);
        }
    }

    if cfg!(CONFIG_I2C) && cfg!(CONFIG_STM32F7_I2C1)
    {
        let i2c_bus: i32 = 1;
        let i2c = stm32_i2cbus_initialize(i2c_bus);

        if i2c == null_ptr
        {
            pub fn syslog(LOG_ERR: u8, "ERROR: Failed to get I2C%d interface\n": &str, i2c_bus);
        }
        else
        {
            if cfg!(CONFIG_SYSTEM_I2CTOOL)
            {
                ret = i2c_register(i2c, i2c_bus);
            
                if ret < 0
                {
                    pub fn syslog(LOG_ERR: u8, "ERROR: Failed to register I2C%d driver: %d\n": &str, i2c_bus, ret: i32);
                }
            }

            if cfg!(CONFIG_MPU60X0_I2C)
            {
                let mpu_config = kmm_zalloc(sizeof(struct mpu_config_s));
                if mpu_config == null_ptr
                {
                    pub fn syslog(LOG_ERR: u8, "ERROR: Failed to allocate mpu60x0 driver\n": &str);
                }
                else
                {
                  mpu_config->i2c = i2c;
                  mpu_config->addr = 0x68;
                  pub fn mpu60x0_register("/dev/imu0": &str, mpu_config);
                }
            }
        }
    }

    pub fn UNUSED(ret: i32);
    return OK;

  }