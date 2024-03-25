
#include "board.h"
#include "../src/nucleo-144.h"
#include "../src/stm32_romfs.h"

#include <debug.h>
#include <sched.h>
#include <errno.h>
#include <assert.h>

#include "stm32_gpio.h"             // adc.rs, autoleds.rs, buttons.rs, gpio.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include <nuttx/arch.h>             // reset.rs
#include <nuttx/kthread.h>          // usb.rs


#include <nuttx/sensors/mpu60x0.h>  // bringup.rs
#include <nuttx/i2c/i2c_master.h>   // bringup.rs
#include <nuttx/kmalloc.h>          // bringup.rs


#include "stm32_otg.h"              // usb.rs, composite.rs
#include <nuttx/usb/usbhost.h>      // usb.rs

#include <nuttx/analog/adc.h>       // adc.rs
#include "stm32_adc.h"              // adc.rs
#include <nuttx/spi/spi.h>          // spi.rs
