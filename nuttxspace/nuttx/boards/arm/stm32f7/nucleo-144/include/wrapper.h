
#include "board.h"
#include "../src/nucleo-144.h"
#include "../src/stm32_romfs.h" // bringup
#include "../../../../arch/arm/src/stm32f7/stm32_gpio.h" // bringup
#include "../../../../../include/nuttx/i2c/i2c_master.h"
#include "../../../../../include/nuttx/sensors/mpu60x0.h"
#include "../../../../../include/nuttx/usb/usbhost.h"// used in stm32_usb for almost all usb calls
#include "../../../../../arch/arm/src/stm32f7/stm32_otg.h" // used in stm32_usb for stm32_otgfshost_initialize && stm32_usbsuspend
#include "../../../../../arch/arm/src/stm32f7/hardware/stm32f74xx75xx_pinmap.h" // used in stm32_usb, for usbhost.h dependencies
#include "../../../../../sched/task/task_create.c" // kthread_create used in stm32_usb.rs
#include "../../../../include/nuttx/board.h"
#include "../../../../include/nuttx/spi/spi.h" // used in stm32_spi -> SPIDEVID_INDEX
#include "../../../../include/debug.h" // used in stm32_spi -> spiinfo
#include "../../../../include/nuttx/analog/adc.h" // used in stm32_adc
#include "../../../../arch/arm/src/stm32f7/hardware/stm32_adc.h" // used in stm32_adc
#include "../../../../arch/arm/src/stm32f7/hardware/stm32f72xx73xx_pinmap_legacy.h" // used in stm32_adc

