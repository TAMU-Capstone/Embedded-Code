
#include "board.h"
#include "../src/nucleo-144.h"
#include "../src/stm32_romfs.h"
#include "../../../../arch/arm/src/stm32f7/stm32_gpio.h"
#include "../../../../include/nuttx/board.h"

/*
#include <nuttx/config.h>

#include <errno.h>
#include <debug.h>

#include <nuttx/board.h>
#include <nuttx/analog/adc.h>
#include <arch/board/board.h>

#include "chip.h"
#include "stm32_gpio.h"
#include "stm32_adc.h"
#include "nucleo-144.h"
*/

// the ones i could find for adc
#include "../../../../include/nuttx/analog/adc.h"
#include "../../../../arch/arm/src/stm32f7/hardware/stm32_adc.h"
#include "../../../../arch/arm/src/stm32f7/hardware/stm32f72xx73xx_pinmap_legacy.h"

