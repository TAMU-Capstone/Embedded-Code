/****************************************************************************
 * apps/testingApp/testingApp_main.c
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

//#include <../../nuttx/include/nuttx/config.h>
#include <nuttx/config.h>
#include <stdio.h>
#include <stdbool.h>
#include <unistd.h>


int GPIO_OUTPUT = 262144;
int GPIO_PUSHPULL = 0;
int GPIO_SPEED_50MHz = 2048;
int OUTPUT_CLEAR = 0;
int GPIO_PIN0 = 0;
int GPIO_PIN7 = 7;
int GPIO_PIN14 = 14;
int GPIO_PORTB = 16;

// #include "stm32_gpio.h"
// #include "nucleo-144.h"
#define GPIO_LD1       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN0)
#define GPIO_LD2       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN7)
#define GPIO_LD3       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN14)

#define GPIO_LED_GREEN GPIO_LD1
#define GPIO_LED_BLUE  GPIO_LD2
#define GPIO_LED_RED   GPIO_LD3


/****************************************************************************
 * Function Definitions
 ****************************************************************************/

// extern int board_userled_initialize(void);
// extern void board_userled(int led, _Bool ledon); //Necessary??????????????????????????///


/****************************************************************************
 * Test Functions
 ****************************************************************************/

void userleds_test(void) {
  int ledset = 0;

  // configure leds
  int NUM_LEDS = board_userled_initialize();

  printf("Test 1 Userleds:\n");
  printf("All three Userleds should be on for 10 seconds.\n");

  // Turn on all three leds
  // for (int i = 0; i < NUM_LEDS; i++) {
  //   board_userled(i, true);
  // }

  // // for (int i = 0; i < NUM_LEDS; i++) {
  // //   board_userled(i, false);
  // // }

  //TESTING*************

  board_userled(0, true); 

  sleep(10);

  board_userled(0, false);

  board_userled(1, true); 

  sleep(10);

  board_userled(1, false);

  board_userled(2, true); 

  sleep(10);

  board_userled(2, false);

  // printf("Green gpio %d\n", GPIO_LED_GREEN);
  // printf("Blue gpio %d\n", GPIO_LED_BLUE);
  // printf("Red gpio %d\n", GPIO_LED_RED);

  // stm32_gpiowrite(GPIO_LED_GREEN, true);
  // sleep(10);
  // stm32_gpiowrite(GPIO_LED_GREEN, false);

  // stm32_gpiowrite(GPIO_LED_BLUE, true);
  // sleep(10);
  // stm32_gpiowrite(GPIO_LED_BLUE, false);

  // stm32_gpiowrite(GPIO_LED_RED, true);
  // sleep(10);
  // stm32_gpiowrite(GPIO_LED_RED, false);
}

/****************************************************************************
 * testingApp_main
 ****************************************************************************/
int main(int argc, FAR char *argv[]) {
  userleds_test();

  return 0;
}
