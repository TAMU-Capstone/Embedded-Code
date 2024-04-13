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

#[allow(unused_imports)]
use core::ptr::{NonNull, null_mut};

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
// extern "C" {
//     pub fn stm32_dma_alloc_init() -> i32;
// }
//main function
//returns an int
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn stm32_bringup() -> cty::c_int {
    //define int _ret
    //define as mutable
    let mut _ret: i32 = 0;

    #[cfg(CONFIG_I2C)]
    {
        let mut i2c_bus: i32;
        let i2c: Option<i2c_master_s>;
        #[cfg(CONFIG_MPU60X0_I2C)]
        let mpu_config: NonNull<mpu_config_s>;
    } // CONFIG_I2C

    #[cfg(CONFIG_FS_PROCFS)]
    {
        /* Mount the procfs file system */
        _ret = unsafe { nx_mount(
            null_mut(),
            STM32_PROCFS_MOUNTPOINT,
            "procfs".as_ptr() as *const u8,
            0,
            null_mut(),
        )}; //null_ptr.as_ptr().cast::<mut *c_void>()
        if _ret < 0 {
            unsafe {
                //let c_str = CString::new(s).unwrap();
                //log_impl(s.as_bytes().as_ptr() as *const u8);
                //https://stackoverflow.com/questions/49203561/how-do-i-convert-a-str-to-a-const-u8 -- converting str to u8
                syslog(
                    LOG_ERR.into(),
                    "ERROR: Failed to mount procfs at %s: %d\n".as_ptr() as *const u8,
                    STM32_PROCFS_MOUNTPOINT,
                    _ret,
                );
            }
        }
    }

    #[cfg(CONFIG_STM32_ROMFS)]
    {
        /* Mount the romfs partition */

        _ret = unsafe{ stm32_romfs_initialize() } ;

        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: Failed to mount romfs at %s: %d\n".as_ptr() as *const u8,
                    CONFIG_STM32_ROMFS_MOUNTPOINT,
                    _ret,
                );
            }
        }
    }

    #[cfg(CONFIG_DEV_GPIO)]
    {
        /* Register the GPIO driver */
        _ret = unsafe{stm32_gpio_initialize()};
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "Failed to initialize GPIO Driver: %d\n".as_ptr() as *const u8,
                    _ret,
                );
                return _ret;
            }
        }
    }

    #[cfg(all(not(CONFIG_ARCH_LEDS), CONFIG_USERLED_LOWER))]
    {
        _ret = userled_lower_initialize(LED_DRIVER_PATH);
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: userled_lower_initialize() failed: {}\n".as_ptr() as *const u8,
                    _ret,
                );
            }
        }
    }

    #[cfg(CONFIG_ADC)]
    {
        _ret = stm32_adc_setup();
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: stm32_adc_setup failed: %d\n".as_ptr() as *const u8,
                    _ret,
                );
            }
        }
    }

    #[cfg(CONFIG_STM32F7_BBSRAM)]
    {
        unsafe {
            stm32_bbsram_int();
        }
    }

    #[cfg(CONFIG_FAT_DMAMEMORY)]
    {
        //TO-DO: may need to use let and make variable and then compare
        unsafe {
            let init = unsafe { stm32_dma_alloc_init() };
            if init < 0 {
                syslog(LOG_ERR.into(), "DMA alloc FAILED".as_ptr() as *const u8);
            }
        }
    }

    #[cfg(CONFIG_NUCLEO_SPI_TEST)]
    {
        _ret = unsafe{stm32_spidev_bus_test()};
        //there is if _ret != OK
        if _ret != OK {
            unsafe{
            syslog(
                LOG_ERR.into(),
                "ERROR: Failed to initialize SPI interfaces: %d\n".as_ptr() as *const u8,
                _ret,
            )};
            return _ret;
        }
    }

    #[cfg(CONFIG_MMCSD)]
    {
        _ret = unsafe{stm32_sdio_initialize()};
        if _ret != OK {
            ferr(
                "ERROR: Failed to initialize MMC/SD driver: %d\n".as_ptr() as *const u8,
                _ret,
            );
            return _ret;
        }
    }

    #[cfg(CONFIG_PWM)]
    {
        _ret = stm32_pwm_setup();
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: stm32_pwm_setup() failed: %d\n".as_ptr() as *const u8,
                    _ret,
                );
            }
        }
    }

    #[cfg(CONFIG_SENSORS_QENCODER)]
    {
        //defines an array of size 9 and initializes it to 0
        let mut buf: [u8; 9] = [0; 9];

        #[cfg(CONFIG_STM32F7_TIM1_QE)]
        {
            //(&buf[0..1]).read_u16::<LittleEndian>();
            unsafe{snprintf(
                buf.as_ptr() as *mut u8,
                buf.len().try_into().unwrap(),
                "/dev/qe0".as_ptr() as *const u8,
            )};
            _ret = unsafe{stm32_qencoder_initialize(buf.into(), 1)};
            if _ret < 0 {
                unsafe{syslog(
                    LOG_ERR.into(),
                    "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8,
                    _ret,
                )};
                return _ret;
            }
        }

        #[cfg(CONFIG_STM32F7_TIM3_QE)]
        {
            unsafe {
                snprintf(
                    buf.as_ptr() as *mut u8,
                    buf.len().try_into().unwrap(),
                    "/dev/qe2".as_ptr() as *const u8,
                ); //.try_into().unwrap()
            }
            unsafe {
                _ret = stm32_qencoder_initialize(buf.as_ptr() as *mut u8, 3);
            }
            if _ret < 0 {
                unsafe {
                    syslog(
                        LOG_ERR.into(),
                        "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8,
                        _ret,
                    );
                    return _ret;
                }
            }
        }

        #[cfg(CONFIG_STM32F7_TIM4_QE)]
        {
            unsafe {
                snprintf(
                    buf.as_ptr() as *mut u8,
                    buf.len().try_into().unwrap(),
                    "/dev/qe3".as_ptr() as *const u8,
                );
            }
            unsafe {
                _ret = stm32_qencoder_initialize(buf.as_ptr() as *mut u8, 4);
            }
            if _ret < 0 {
                unsafe {
                    syslog(
                        LOG_ERR.into(),
                        "ERROR: Failed to register the qencoder: %d\n".as_ptr() as *const u8,
                        _ret,
                    );
                    return _ret;
                }
            }
        }
    } // CONFIG_SENSORS_QENCODER

    #[cfg(CONFIG_STM32F7_CAN_CHARDRIVER)]
    {
        _ret = unsafe{stm32_can_setup()};
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: stm32f7_can_setup failed: %d\n".as_ptr() as *const u8,
                    _ret,
                );
            }
            return _ret;
        }
    }

    #[cfg(CONFIG_STM32F7_CAN_SOCKET)]
    {
        unsafe {
            _ret = stm32_cansock_setup();
        }
        if _ret < 0 {
            unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: stm32_cansock_setup failed: %d\n".as_ptr() as *const u8,
                    _ret,
                );
            }
        }
    }

    #[cfg(all(CONFIG_I2C, CONFIG_STM32F7_I2C1))] {
        let i2c_bus: i32 = 1;
        let _i2c: Option<_> = NonNull::<i2c_master_s>::new(unsafe { stm32_i2cbus_initialize(i2c_bus) });
        match _i2c {
            None => unsafe {
                syslog(
                    LOG_ERR.into(),
                    "ERROR: Failed to get I2C%d interface\n".as_ptr() as *const u8,
                    i2c_bus,
                );
            },
            Some(_i2c) => {
                #[cfg(CONFIG_SYSTEM_I2CTOOL)]
                {
                    _ret = unsafe{ i2c_register(_i2c.as_ptr(), i2c_bus) };

                    if _ret < 0 {
                        unsafe {
                            syslog(
                                LOG_ERR.into(),
                                "ERROR: Failed to register I2C%d driver: %d\n".as_ptr(),
                                i2c_bus,
                                _ret,
                            );
                        }
                    }
                }
                #[cfg(CONFIG_MPU60X0_I2C)]
                {
                    //let mpu_config = unsafe{kmm_zalloc(get_size(mpu_config_s))};
                    //kmm_zalloc calls zalloc()
                    let mpu_config: Option<_> = NonNull::<mpu_config_s>::new(unsafe { zalloc(get_size(mpu_config_s)) });

                    match mpu_config {
                        None => unsafe{
                            syslog(
                                LOG_ERR.into(),
                                "ERROR: Failed to allocate mpu60x0 driver\n".as_ptr() as *const u8,
                            );
                        },
                        Some(mpu_config) => unsafe {
                            mpu_config.as_ref().i2c = _i2c.as_ptr();
                            mpu_config.as_ref().addr = 0x68;
                            mpu60x0_register("/dev/imu0".as_ptr() as *const u8, mpu_config.as_ptr());
                        }
                    }
                }
            }
        }
    }
    return OK;
}
