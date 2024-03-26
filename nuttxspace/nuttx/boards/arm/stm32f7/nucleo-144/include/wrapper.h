/**
 * Written by Cole McAnelly
 * 
 * For Bingen to parse and create Rust FFI bindings to interact with NuttX operating system
 * Created by using this pipeline command:
 * 
 * 
grep "#include" src/*.c | awk '
    BEGIN{FS=":"; OFS="\t// "};
    { arr[$2] = arr[$2] == ""? $1 : arr[$2] ", " $1 }
    END {for (i in arr) print i, arr[i] }
' \
| sort \
| sed 's/\.c/\.rs/g' \
| sed 's@src/stm32_@@g';
*/

<<<<<<< HEAD
#include "board.h"
#include "../src/nucleo-144.h"
#include "../src/stm32_romfs.h"

#include <debug.h>
#include <sched.h>
#include <errno.h>
#include <assert.h>

<<<<<<< HEAD
// #include "../../../../../include/nuttx/i2c/i2c_master.h"
// #include "stm32_otg.h" // used in stm32_usb for stm32_otgfshost_initialize && stm32_usbsuspend
// #include "../../../../../include/nuttx/sensors/mpu60x0.h"
// #include "../../../../../include/nuttx/usb/usbhost.h"// used in stm32_usb for almost all usb calls
#include "../../../../../arch/arm/src/stm32f7/hardware/stm32f74xx75xx_pinmap.h" // used in stm32_usb, for usbhost.h dependencies
<<<<<<< HEAD
// #include "../../../../include/nuttx/board.h"
// #include "../../../../include/nuttx/spi/spi.h" // used in stm32_spi -> SPIDEVID_INDEX
// #include "../../../../include/debug.h" // used in stm32_spi -> spiinfo
// #include "../../../../include/nuttx/analog/adc.h" // used in stm32_adc
// #include "../../../../arch/arm/src/stm32f7/hardware/stm32_adc.h" // used in stm32_adc
// #include "../../../../arch/arm/src/stm32f7/hardware/stm32f72xx73xx_pinmap_legacy.h" // used in stm32_adc
// #include "../../../../../include/nuttx/lib/stdarg.h" //used in usb for struct declaration
=======
#include "../../../../../sched/task/task_create.c" // kthread_create used in stm32_usb.rs
#include "../../../../include/nuttx/board.h"
#include "../../../../include/nuttx/spi/spi.h" // used in stm32_spi -> SPIDEVID_INDEX
#include "../../../../include/debug.h" // used in stm32_spi -> spiinfo
#include "../../../../include/nuttx/analog/adc.h" // used in stm32_adc
#include "../../../../arch/arm/src/stm32f7/hardware/stm32_adc.h" // used in stm32_adc
#include "../../../../arch/arm/src/stm32f7/hardware/stm32f72xx73xx_pinmap_legacy.h" // used in stm32_adc
#include "../../../../../include/nuttx/lib/stdarg.h" //used in usb for struct declaration
#include "../../../../arch/arm64/include/stdarg.h" // used in stdarg.h for va list declaration 
>>>>>>> 2bcd2318 (asdfasdf)
=======
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
>>>>>>> 3abba875 (Reorginized wrapper.h and included several necessary deps for building)
=======
#include "arm_internal.h"               // bbsram.rs, boot.rs, pwm.rs, qencoder.rs, spi.rs, usb.rs
#include "chip.h"                       // adc.rs, gpio.rs, pwm.rs, qencoder.rs, sdio.rs, spi.rs, usb.rs
#include "nucleo-144.h"                 // adc.rs, appinitialize.rs, autoleds.rs, bbsram.rs, boot.rs, bringup.rs, buttons.rs, can.rs, dma_alloc.rs, gpio.rs, pwm.rs, qencoder.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include "stm32_adc.h"                  // adc.rs
#include "stm32_bbsram.h"               // bbsram.rs
#include "stm32_can.h"                  // can.rs, cansock.rs
#include "stm32_gpio.h"                 // adc.rs, autoleds.rs, buttons.rs, gpio.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include "stm32_i2c.h"                  // bringup.rs
#include "stm32_otg.h"                  // composite.rs, usb.rs
#include "stm32_pwm.h"                  // pwm.rs
#include "stm32_qencoder.h"             // qencoder.rs
#include "stm32_romfs.h"                // bringup.rs, romfs_initialize.rs
#include "stm32_sdmmc.h"                // sdio.rs
#include "stm32_spi.h"                  // spi.rs
#include <arch/board/board.h>           // adc.rs, autoleds.rs, boot.rs, buttons.rs, gpio.rs, pwm.rs, qencoder.rs, spi.rs, userleds.rs
#include <assert.h>                     // bbsram.rs, composite.rs, gpio.rs, usb.rs
#include <debug.h>                      // adc.rs, autoleds.rs, bbsram.rs, boot.rs, bringup.rs, can.rs, cansock.rs, composite.rs, gpio.rs, pwm.rs, qencoder.rs, romfs_initialize.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include <errno.h>                      // adc.rs, bbsram.rs, buttons.rs, can.rs, dma_alloc.rs, pwm.rs, qencoder.rs, romfs_initialize.rs, sdio.rs, spi.rs, usb.rs
#include <fcntl.h>                      // bbsram.rs
#include <nuttx/analog/adc.h>           // adc.rs
#include <nuttx/arch.h>                 // reset.rs
#include <nuttx/board.h>                // adc.rs, autoleds.rs, boot.rs, buttons.rs, pwm.rs, reset.rs, userleds.rs
#include <nuttx/can/can.h>              // can.rs
#include <nuttx/clock.h>                // gpio.rs
#include <nuttx/config.h>               // adc.rs, appinitialize.rs, autoleds.rs, bbsram.rs, boot.rs, bringup.rs, buttons.rs, can.rs, cansock.rs, composite.rs, dma_alloc.rs, gpio.rs, pwm.rs, qencoder.rs, reset.rs, romfs_initialize.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include <nuttx/drivers/ramdisk.h>      // romfs_initialize.rs
#include <nuttx/fs/fs.h>                // bbsram.rs, bringup.rs, romfs_initialize.rs
#include <nuttx/i2c/i2c_master.h>       // bringup.rs
#include <nuttx/ioexpander/gpio.h>      // gpio.rs
#include <nuttx/irq.h>                  // buttons.rs
#include <nuttx/kmalloc.h>              // bringup.rs
#include <nuttx/kthread.h>              // usb.rs
#include <nuttx/leds/userled.h>         // bringup.rs
#include <nuttx/mm/gran.h>              // dma_alloc.rs
#include <nuttx/mmcsd.h>                // sdio.rs
#include <nuttx/sdio.h>                 // sdio.rs
#include <nuttx/sensors/mpu60x0.h>      // bringup.rs
#include <nuttx/sensors/qencoder.h>     // qencoder.rs
#include <nuttx/spi/spi.h>              // spi.rs
#include <nuttx/timers/pwm.h>           // pwm.rs
#include <nuttx/usb/cdcacm.h>           // composite.rs
#include <nuttx/usb/composite.h>        // composite.rs
#include <nuttx/usb/rndis.h>            // composite.rs
#include <nuttx/usb/usbdev.h>           // composite.rs, usb.rs
#include <nuttx/usb/usbdev_trace.h>     // usb.rs
#include <nuttx/usb/usbhost.h>          // usb.rs
#include <nuttx/usb/usbmsc.h>           // composite.rs
#include <nuttx/wdog.h>                 // gpio.rs
#include <sched.h>                      // usb.rs
#include <stdbool.h>                    // autoleds.rs, can.rs, gpio.rs, sdio.rs, spi.rs, usb.rs, userleds.rs
#include <stddef.h>                     // bbsram.rs, buttons.rs
#include <stdint.h>                     // bbsram.rs, dma_alloc.rs, romfs_initialize.rs, spi.rs, usb.rs
#include <stdio.h>                      // bringup.rs, sdio.rs
#include <stdlib.h>                     // bbsram.rs
#include <string.h>                     // bbsram.rs
#include <sys/ioctl.h>                  // bbsram.rs
#include <sys/mount.h>                  // romfs_initialize.rs
#include <sys/param.h>                  // autoleds.rs, bbsram.rs, spi.rs, userleds.rs
#include <sys/types.h>                  // appinitialize.rs, bringup.rs, composite.rs, pwm.rs, romfs_initialize.rs, usb.rs
#include <syslog.h>                     // bbsram.rs, bringup.rs, dma_alloc.rs
>>>>>>> b7a4957f (Modified wrapper.h to include every possible dependancy for every file we will convert in the future)
