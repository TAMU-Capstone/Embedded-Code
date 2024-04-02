/**
 * Written by Cole McAnelly
 * 
 * For Bingen to parse and create Rust FFI bindings to interact with NuttX operating system
 * Created by using this pipeline command:
 * 
 * 
 */
// grep "#include" src/*.c | awk '
//     BEGIN{FS=":"; OFS="\t// "};
//     { arr[$2] = arr[$2] == ""? $1 : arr[$2] ", " $1 }
//     END {for (i in arr) print i, arr[i] }
// ' \
// | sort \
// | sed 's/\.c/\.rs/g' \
// | sed 's@src/stm32_@@g';





#include "stm32_otg.h"              // usb.rs, composite.rs
#include <nuttx/usb/usbhost.h>      // usb.rs

#include <nuttx/analog/adc.h>       // adc.rs
#include "stm32_adc.h"              // adc.rs
#include <nuttx/spi/spi.h>          // spi.rs
#include <nuttx/sensors/mpu60x0.h>  // bringup.rs
#include <nuttx/i2c/i2c_master.h>   // bringup.rs
#include <nuttx/kmalloc.h>          // bringup.rs
#include <nuttx/leds/userled.h>     // bringup.rs
#include <nuttx/fs/fs.h>            // bringup.rs
#include "stm32_i2c.h"      // bringup.rs
#include <syslog.h>
#include <stdio.h>
#include <nuttx/config.h>
