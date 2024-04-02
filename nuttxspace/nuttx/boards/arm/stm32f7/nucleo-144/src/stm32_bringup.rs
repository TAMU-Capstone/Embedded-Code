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
//pub use generated::*;
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
extern "C" {
    pub fn stm32_dma_alloc_init() -> i32;
}
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
    //if cfg!(CONFIG_I2C)
    #[cfg(CONFIG_I2C)]
    {
        let mut i2c_bus: i32;
        //should I use Box? -- its a smart pointer
        let mut i2c = &mut i2c_master_s { ops: val };

        //if cfg!(CONFIG_MPU60X0_I2C)
        #[cfg(CONFIG_MPU60X0_I2C)]
        {
            let mut mpu_config = &mut mpu_config_s { i2c: val, addr: val };
        }
    } // CONFIG_I2C

    //if cfg!(CONFIG_FS_PROCFS)
    #[cfg(CONFIG_FS_PROCFS)]
    {
        /* Mount the procfs file system */
        ret = nx_mount(null_ptr, STM32_PROCFS_MOUNTPOINT, "procfs".as_ptr() as *const u8, 0, core::ptr::null_mut()); //null_ptr.as_ptr().cast::<mut *c_void>()
        if ret < 0
        {
            unsafe{
                //let c_str = CString::new(s).unwrap();
                //log_impl(s.as_bytes().as_ptr() as *const u8);
                //https://stackoverflow.com/questions/49203561/how-do-i-convert-a-str-to-a-const-u8 -- converting str to u8
                syslog(LOG_ERR.into(), "ERROR: Failed to mount procfs at %s: %d\n".as_ptr() as *const u8, STM32_PROCFS_MOUNTPOINT, ret);
            }
        }
    }

    //if cfg!(CONFIG_STM32_ROMFS)
    #[cfg(CONFIG_STM32_ROMFS)]
    {
        /* Mount the romfs partition */

        ret = stm32_romfs_initialize();

        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: Failed to mount romfs at %s: %d\n".as_ptr() as *const u8, CONFIG_STM32_ROMFS_MOUNTPOINT, ret);
            }
        }
    }

    //if cfg!(CONFIG_DEV_GPIO)
    #[cfg(CONFIG_DEV_GPIO)]
    {
        /* Register the GPIO driver */
        ret = stm32_gpio_initialize();
        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "Failed to initialize GPIO Driver: %d\n".as_ptr() as *const u8, ret);
                return ret;
            }
        }
    }

    //if cfg!(not(CONFIG_ARCH_LEDS)) && cfg!(CONFIG_USERLED_LOWER)
    #[cfg(not(CONFIG_ARCH_LEDS))]
    {
        #[(CONFIG_USERLED_LOWER)]
        {
            ret = userled_lower_initialize(LED_DRIVER_PATH);
            if ret < 0
            {
                unsafe{
                    syslog(LOG_ERR.into(), "ERROR: userled_lower_initialize() failed: {}\n".as_ptr() as *const u8, ret);
                }
            }
        }
    }

    //if cfg!(CONFIG_ADC)
    #[cfg(CONFIG_ADC)]
    {
        ret = stm32_adc_setup();
        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: stm32_adc_setup failed: %d\n".as_ptr() as *const u8, ret);
            }
        }
    }

    //if cfg!(CONFIG_STM32F7_BBSRAM)
    #[cfg(CONFIG_STM32F7_BBSRAM)]
    {
        unsafe{
            stm32_bbsram_int();
        }
    }

    //if cfg!(CONFIG_FAT_DMAMEMORY)
    #[cfg(CONFIG_FAT_DMAMEMORY)]
    {
        //TO-DO: may need to use let and make variable and then compare
        unsafe
        {
            let init = unsafe{
                stm32_dma_alloc_init()
            };
            if init < 0
            {
                syslog(LOG_ERR.into(), "DMA alloc FAILED".as_ptr() as *const u8);
            }
        }
    }

    //if cfg!(CONFIG_NUCLEO_SPI_TEST)
    #[cfg(CONFIG_NUCLEO_SPI_TEST)]
    {
        ret = stm32_spidev_bus_test();
        //there is if ret != OK
        if ret != OK
        {
            syslog(LOG_ERR.into(), "ERROR: Failed to initialize SPI interfaces: %d\n".as_ptr() as *const u8, ret);
            return ret;
        }
    }

    //if cfg!(CONFIG_MMCSD)
    #[cfg(CONFIG_MMCSD)]
    {
        ret = stm32_sdio_initialize();
        if ret != OK
        {
            ferr("ERROR: Failed to initialize MMC/SD driver: %d\n".as_ptr() as *const u8, ret);
            return ret;
        }
    }

    //if cfg!(CONFIG_PWM)
    #[cfg(CONFIG_PWM)]
    {
        ret = stm32_pwm_setup();
        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: stm32_pwm_setup() failed: %d\n".as_ptr() as *const u8, ret);
            }
        }
    }

    //if cfg!(CONFIG_SENSORS_QENCODER)
    #[cfg(CONFIG_SENSORS_QENCODER)]
    {
        //defines an array of size 9 and initializes it to 0
        let mut buf : [u8; 9] = [0; 9];
        
        //if cfg!(CONFIG_STM32F7_TIM1_QE)
        #[cfg(CONFIG_STM32F7_TIM1_QE)]
        {
            //(&buf[0..1]).read_u16::<LittleEndian>();
            snprintf(buf.as_ptr() as *mut u8, buf.len().try_into().unwrap(), "/dev/qe0".as_ptr() as *const u8);
            ret = stm32_qencoder_initialize(buf.into(), 1);
            if ret < 0
            {
                syslog(LOG_ERR.into(), "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8, ret);
                return ret;
            }
        }
    
        //if cfg!(CONFIG_STM32F7_TIM3_QE)
        #[cfg(CONFIG_STM32F7_TIM3_QE)]
        {
            unsafe{
                snprintf(buf.as_ptr() as *mut u8, buf.len().try_into().unwrap(), "/dev/qe2".as_ptr() as *const u8);  //.try_into().unwrap()
            }
            unsafe{
                ret = stm32_qencoder_initialize(buf.as_ptr() as *mut u8, 3);
            }
            if ret < 0
            {
                unsafe{
                    syslog(LOG_ERR.into(), "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8, ret);
                    return ret;
                }
            }
        }

        //if cfg!(CONFIG_STM32F7_TIM4_QE)
        #[cfg(CONFIG_STM32F7_TIM4_QE)]
        {
            unsafe{
                snprintf(buf.as_ptr() as *mut u8, buf.len().try_into().unwrap(), "/dev/qe3".as_ptr() as *const u8);
            }
            unsafe{
                ret = stm32_qencoder_initialize(buf.as_ptr() as *mut u8, 4);
            }
            if ret < 0
            {
                unsafe{
                    syslog(LOG_ERR.into(), "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8, ret);
                    return ret;
                }
            }
        }
    } // CONFIG_SENSORS_QENCODER
    
    //if cfg!(CONFIG_STM32F7_CAN_CHARDRIVER)
    #[cfg(CONFIG_STM32F7_CAN_CHARDRIVER)]
    {
        ret = stm32_can_setup();
        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: stm32f7_can_setup failed: %d\n".as_ptr() as *const u8, ret);
            }
            return ret;
        }
    }

    //if cfg!(CONFIG_STM32F7_CAN_SOCKET)
    #[cfg(CONFIG_STM32F7_CAN_SOCKET)]
    {
        unsafe{
            ret = stm32_cansock_setup();
        }
        if ret < 0
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: stm32_cansock_setup failed: %d\n".as_ptr() as *const u8, ret);
            }
        }
    }

    if cfg!(CONFIG_I2C) && cfg!(CONFIG_STM32F7_I2C1)
    {
        let mut i2c_bus : i32 = 1;
        let mut i2c = stm32_i2cbus_initialize(i2c_bus);

        if i2c.into() == null_ptr
        {
            unsafe{
                syslog(LOG_ERR.into(), "ERROR: Failed to get I2C%d interface\n".as_ptr() as *const u8, i2c_bus);
            }
        }
        else
        {
            //if cfg!(CONFIG_SYSTEM_I2CTOOL)
            #[cfg(CONFIG_SYSTEM_I2CTOOL)]
            {
                ret = i2c_register(i2c, i2c_bus);
            
                if ret < 0
                {
                    unsafe{
                        syslog(LOG_ERR.into(), "ERROR: Failed to register I2C%d driver: %d\n".as_ptr() as *const u8, i2c_bus, ret);
                    }
                }
            }

            //if cfg!(CONFIG_MPU60X0_I2C)
            #[cfg(CONFIG_MPU60X0_I2C)]
            {
                let mpu_config = kmm_zalloc(get_size(mpu_config_s));
                if mpu_config == null_ptr
                {
                    unsafe{
                        syslog(LOG_ERR.into(), "ERROR: Failed to allocate mpu60x0 driver\n".as_ptr() as *const u8);
                    }
                }
                else
                {
                  mpu_config.i2c = i2c;
                  mpu_config.addr = 0x68;
                  unsafe{
                        mpu60x0_register("/dev/imu0".as_ptr() as *const u8, mpu_config);
                  }
                }
            }
        }
    }
    unsafe
    {
        UNUSED(ret);
    }
    return OK;
}