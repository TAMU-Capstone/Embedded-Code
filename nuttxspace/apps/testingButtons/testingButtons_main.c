/****************************************************************************
 * apps/testingButtons/testingButtons_main.c
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
#include <stdio.h>
#include <stddef.h>
#include <stdbool.h>
#include <errno.h>

/****************************************************************************
 * Function Definitions
 ****************************************************************************/
int board_button_irq(int id, xcpt_t irqhandler, void *arg);
uint32_t board_button_initialize(void);
uint32_t board_buttons(void);
/****************************************************************************
 * Test Functions
 * in board.h
 * #define BUTTON_USER        0
 * #define NUM_BUTTONS        1
 * #define BUTTON_USER_BIT    (1 << BUTTON_USER)
 ****************************************************************************/

// most of how to use irqhandler was derrived from this logic here
// https://github.com/nodesign/nuttx-apps/blob/master/examples/buttons/buttons_main.c

void testingButtons(void)
{
    printf("testing Buttons\n");
    board_button_initialize();
    printf("This board only has one programmable button (blue) this is the user button\n");
    printf("You will have 3s [calling sleep 3] to press and hold the user button, and we will read the buttons state after\n");
    uint32_t buttonState = -1;
    sleep(3);
    buttonState = board_buttons();
    printf("User button is currently %lu \n", buttonState);
    buttonState = -1;
#ifdef CONFIG_ARCH_IRQBUTTONS
    xcpt_t handler; /* Button interrupt handler this can be more complicated and better if needed, see github resource above */
    printf("setting interupt handling to flash led on user button (blue) press");
    printf(" if -22, invalid arguemnet = something failed");
    printf("test: %d", board_button_irq(0, handler, NULL));
    printf(" while looping to 1k you can press button ");
    for (int i = 0; i < 1000; i++){}
#endif
#ifndef CONFIG_ARCH_IRQBUTTONS
        printf("interupt option has not been defined in menu config!");
#endif
}

int main(int argc, FAR char *argv[])
{
    testingButtons();

    return 0;
}
