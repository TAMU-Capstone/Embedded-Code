
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
