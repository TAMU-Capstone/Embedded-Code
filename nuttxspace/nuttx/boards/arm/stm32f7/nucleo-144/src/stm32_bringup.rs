/**
 * Rewritten in Rust by Cole McAnelly and Isidora Wright
 */
/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*;
use cty;
use crate::err;

#[allow(unused_imports)]
use core::ptr::NonNull;

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
#[no_mangle]
pub extern "C" fn stm32_bringup() -> cty::c_int {
    /* Mount the procfs file system */
    #[cfg(CONFIG_FS_PROCFS)]
    match unsafe {
        nx_mount(
            core::ptr::null_mut(),
            STM32_PROCFS_MOUNTPOINT,
            "procfs".as_ptr() as *const i8,
            0,
            core::ptr::null_mut(),
        )
    } {
        OK => (),
        ret => err!(
            "ERROR: Failed to mount procfs at %s: %d\n",
            STM32_PROCFS_MOUNTPOINT,
            ret
        ),
    }

    /* Mount the romfs partition */
    #[cfg(CONFIG_STM32_ROMFS)]
    match unsafe { stm32_romfs_initialize() } {
        OK => (),
        ret => err!(
            "ERROR: Failed to mount romfs at %s: %d\n",
            CONFIG_STM32_ROMFS_MOUNTPOINT,
            ret
        ),
    }

    /* Register the GPIO driver */
    #[cfg(CONFIG_DEV_GPIO)]
    match unsafe { stm32_gpio_initialize() } {
        OK => (),
        ret => {
            err!("Failed to initialize GPIO Driver: %d\n", ret);
            return ret;
        }
    }

    #[cfg(all(not(CONFIG_ARCH_LEDS), CONFIG_USERLED_LOWER))]
    match unsafe { userled_lower_initialize(LED_DRIVER_PATH) } {
        OK => (),
        ret => err!("ERROR: userled_lower_initialize() failed: {}\n", ret),
    }

    #[cfg(CONFIG_ADC)]
    match unsafe { stm32_adc_setup() } {
        OK => (),
        ret => err!("ERROR: stm32_adc_setup failed: %d\n", ret),
    }

    #[cfg(CONFIG_STM32F7_BBSRAM)]
    unsafe { stm32_bbsram_int() };

    #[cfg(CONFIG_FAT_DMAMEMORY)]
    if unsafe { stm32_dma_alloc_init() } < 0 {
        err!("DMA alloc FAILED");
    }

    #[cfg(CONFIG_NUCLEO_SPI_TEST)]
    match unsafe { stm32_spidev_bus_test() } {
        OK => (),
        ret => {
            err!("ERROR: Failed to initialize SPI interfaces: %d\n", ret);
            return ret;
        }
    }

    #[cfg(CONFIG_MMCSD)]
    match unsafe { stm32_sdio_initialize() } {
        OK => (),
        ret => {
            err!("ERROR: Failed to initialize MMC/SD driver: %d\n", ret);
            return ret;
        }
    }

    #[cfg(CONFIG_PWM)]
    match unsafe { stm32_pwm_setup() } {
        OK => (),
        ret => err!("ERROR: stm32_pwm_setup() failed: %d\n", ret),
    }

    //defines an array of size 9 and initializes it to 0
    #[cfg(CONFIG_SENSORS_QENCODER)]
    let mut buf: [i8; 9] = [0; 9];

    #[cfg(all(CONFIG_STM32F7_TIM1_QE, CONFIG_SENSORS_QENCODER))]
    {
        let ret = unsafe {
            snprintf(
                buf.as_mut_ptr(),
                buf.len() as u32,
                "/dev/qe0".as_ptr() as *const i8,
            );
            stm32_qencoder_initialize(buf.into(), 1)
        };
        if ret < 0 {
            err!("ERROR: Failed to register the qencoder: %d\n", ret);
            return ret;
        }
    }

    #[cfg(all(CONFIG_STM32F7_TIM3_QE, CONFIG_SENSORS_QENCODER))]
    {
        let ret = unsafe {
            snprintf(
                buf.as_mut_ptr(),
                buf.len() as u32,
                "/dev/qe2".as_ptr() as *const i8,
            );
            stm32_qencoder_initialize(buf.into(), 3)
        };
        if ret < 0 {
            err!("ERROR: Failed to register the qencoder: %d\n", ret);
            return ret;
        }
    }

    #[cfg(all(CONFIG_STM32F7_TIM4_QE, CONFIG_SENSORS_QENCODER))]
    {
        let ret = unsafe {
            snprintf(
                buf.as_mut_ptr(),
                buf.len() as u32,
                "/dev/qe3".as_ptr() as *const i8,
            );
            stm32_qencoder_initialize(buf.into(), 4)
        };
        if ret < 0 {
            err!("ERROR: Failed to register the qencoder: %d\n", ret);
            return ret;
        }
    }

    #[cfg(CONFIG_STM32F7_CAN_CHARDRIVER)]
    match unsafe { stm32_can_setup() } {
        OK => (),
        ret => {
            err!("ERROR: stm32f7_can_setup failed: %d\n", ret);
            return ret;
        }
    }

    #[cfg(CONFIG_STM32F7_CAN_SOCKET)]
    match unsafe { stm32_cansock_setup() } {
        OK => (),
        ret => {
            err!("ERROR: stm32_cansock_setup failed: %d\n", ret);
            return ret;
        }
    }

    #[cfg(all(CONFIG_I2C, CONFIG_STM32F7_I2C1))]
    let i2c_bus: i32 = 1;

    #[cfg(all(CONFIG_I2C, CONFIG_STM32F7_I2C1))]
    match NonNull::<i2c_master_s>::new(unsafe { stm32_i2cbus_initialize(i2c_bus) }) {
        None => err!("ERROR: Failed to get I2C%d interface\n", i2c_bus),
        Some(mut _i2c) => {
            #[cfg(CONFIG_SYSTEM_I2CTOOL)]
            match unsafe { i2c_register(_i2c.as_ptr(), i2c_bus) } {
                OK => (),
                ret => err!("ERROR: Failed to register I2C%d driver: %d\n", i2c_bus, ret),
            }

            #[cfg(CONFIG_MPU60X0_I2C)]
            match NonNull::new(unsafe {
                zalloc(core::mem::size_of::<mpu_config_s>()) as *mut mpu_config_s
            }) {
                None => err!("ERROR: Failed to allocate mpu60x0 driver\n"),
                Some(mut mpu_config) => unsafe {
                    mpu_config.as_mut().i2c = _i2c.as_ptr();
                    mpu_config.as_mut().addr = 0x68;
                    mpu60x0_register("/dev/imu0".as_ptr() as *const i8, mpu_config.as_ptr());
                }
            }
        }
    }
    return OK;
}
