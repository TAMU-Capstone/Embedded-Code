/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_buttons.c
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

#include <nuttx/config.h>

#include <stddef.h>
#include <errno.h>

#include <nuttx/irq.h>
#include <nuttx/board.h>

#include <arch/board/board.h>

#include "stm32_gpio.h"
#include "nucleo-144.h"



/****************************************************************************
* Test Functions
 * in board.h
 * #define BUTTON_USER        0
 * #define NUM_BUTTONS        1
 * #define BUTTON_USER_BIT    (1 << BUTTON_USER)
 ****************************************************************************/
#define CONFIG_ARCH_BUTTONS; // make sure that we can call these C functions
#define CONFIG_ARCH_IRQBUTTONS;



void testingButtons(void)
{
  uint32_t numberOfButtons  = -1;

  printf("testing Buttons\n");
  numberOfButtons = board_button_initialize();
  printf("number of buttons %zu \n", numberOfButtons);

  printf("for each buttonn what is it high or low?\n");
  printf("buttonState is set -1 each iteration\n");
  uint32_t buttonState = -1;
  for (uint32_t i = 0; i < numberOfButtons; i++)
  {
    buttonState = board_button_initialize();
    printf("button number %zu is currently %zu \n", i, buttonState);
    buttonState = -1;
  }

  printf("setting interupt handling to flash led on user button (blue) press");
  xcpt_t exceptionHandler = 
  printf(" if -22, invalid arguemnet = something failed");
  printf("test: %n",board_button_irq(0, exceptionHandler, void));

  

}

/****************************************************************************
 * Button support.
 *
 * Description:
 *   board_button_initialize() must be called to initialize button resources.
 *   After that, board_buttons() may be called to collect the current state
 *   of all buttons or board_button_irq() may be called to register button
 *   interrupt handlers.
 *
 *   After board_button_initialize() has been called, board_buttons() may be
 *   called to collect the state of all buttons.  board_buttons() returns an
 *   32-bit bit set with each bit associated with a button.  See the
 *   BUTTON_*_BIT definitions in board.h for the meaning of each bit.
 *
 *   board_button_irq() may be called to register an interrupt handler that
 *   will be called when a button is depressed or released. The ID value is a
 *   button enumeration value that uniquely identifies a button resource. See
 *   the BUTTON_* definitions in board.h for the meaning of enumeration
 *   value.
 *
 ****************************************************************************/

int board_button_irq(int id, xcpt_t irqhandler, void *arg)
{
  int ret = -EINVAL;

  if (id == BUTTON_USER)
    {
      ret = stm32_gpiosetevent(GPIO_BTN_USER, true, true, true,
                               irqhandler, arg);
    }

  return ret;
}
