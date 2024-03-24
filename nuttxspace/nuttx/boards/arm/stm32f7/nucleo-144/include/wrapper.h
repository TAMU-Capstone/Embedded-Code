
#include "board.h"
#include "../src/nucleo-144.h"
#include "../src/stm32_romfs.h"
#include "../../../../arch/arm/src/stm32f7/stm32_gpio.h"
#include "../../../../../include/nuttx/usb/usbhost.h"// used in stm32_usb for almost all usb calls
#include "../../../../../arch/arm/src/stm32f7/stm32_otg.h" // used in stm32_usb for stm32_otgfshost_initialize && stm32_usbsuspend
#include "../../../../../arch/arm/src/stm32f7/hardware/stm32f74xx75xx_pinmap.h" // used in stm32_usb, for usbhost.h dependencies
#include "../../../../../sched/task/task_create.c" // kthread_create used in stm32_usb.rs