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

#define LED_STARTED        0 /* NuttX has been started   OFF    OFF   OFF  */
#define LED_HEAPALLOCATE   1 /* Heap has been allocated  OFF    OFF   ON   */
#define LED_IRQSENABLED    2 /* Interrupts enabled       OFF    ON    OFF  */
#define LED_STACKCREATED   3 /* Idle stack created       OFF    ON    ON   */
#define LED_INIRQ          4 /* In an interrupt          N/C    N/C   GLOW */
#define LED_SIGNAL         5 /* In a signal handler      N/C    GLOW  N/C  */
#define LED_ASSERTION      6 /* An assertion failed      GLOW   N/C   GLOW */
#define LED_PANIC          7 /* The system has crashed   Blink  OFF   N/C  */
#define LED_IDLE           8 /* MCU is is sleep mode     ON     OFF   OFF  */


// int GPIO_OUTPUT = 262144;
// int GPIO_PUSHPULL = 0;
// int GPIO_SPEED_50MHz = 2048;
// int OUTPUT_CLEAR = 0;
// int GPIO_PIN0 = 0;
// int GPIO_PIN7 = 7;
// int GPIO_PIN14 = 14;
// int GPIO_PORTB = 16;

// #include "stm32_gpio.h"
// #include "nucleo-144.h"
// #define GPIO_LD1       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN0)
// #define GPIO_LD2       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN7)
// #define GPIO_LD3       (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | OUTPUT_CLEAR | GPIO_PORTB | GPIO_PIN14)

// #define GPIO_LED_GREEN GPIO_LD1
// #define GPIO_LED_BLUE  GPIO_LD2
// #define GPIO_LED_RED   GPIO_LD3


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
  for (int i = 0; i < NUM_LEDS; i++) {
    board_userled(i, true);
  }

  sleep(10);

  for (int i = 0; i < NUM_LEDS; i++) {
    board_userled(i, false);
  }

  //TESTING*************

  // board_userled(0, true); 


  // sleep(3);

  // board_userled(0, false);

  // board_userled(1, true); 


  // sleep(3);

  // board_userled(1, false);

  // board_userled(2, true); 
  

  // sleep(3);

  // board_userled(2, false);

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

void autoleds_test(void) {
  int NUM_LEDS = board_userled_initialize();

  int OnParams[8] = {LED_HEAPALLOCATE, LED_IRQSENABLED, LED_STACKCREATED, LED_INIRQ, LED_SIGNAL, LED_ASSERTION, LED_PANIC, LED_IDLE};
  
  char OnMessages[8][60];
  strcpy(OnMessages[0], "LED_HEAPALLOCATE turns on blue");
  strcpy(OnMessages[1], "LED_IRQSENABLED turns off blue and turns on green");
  strcpy(OnMessages[2], "LED_STACKCREATED turns on green and blue");
  strcpy(OnMessages[3], "LED_INIRQ turns on blue");
  strcpy(OnMessages[4], "LED_SIGNAL turns on green");
  strcpy(OnMessages[5], "LED_ASSERTION turns on red and blue");
  strcpy(OnMessages[6], "LED_PANIC turns on red");
  strcpy(OnMessages[7], "LED_IDLE turns on red");

  //int OffParams[5] = [LED_SIGNAL, LED_INIRQ, LED_ASSERTION, LED_PANIC, LED_IDLE];

  printf("Test 2 Autoleds:\n");

  for (int i = 0; i < 8; i++) {
    printf("%s\n", OnMessages[i]);
    board_autoled_on(OnParams[i]);
    sleep(5);

    //These two parameters turn off all 3 leds (Resetting all lights to zero after each loop)
    board_autoled_off(LED_SIGNAL);
    board_autoled_off(LED_ASSERTION);
    sleep(2);
  }
}

/****************************************************************************
 * testingApp_main
 ****************************************************************************/
int main(int argc, FAR char *argv[]) {
  //userleds_test();
  autoleds_test();

  return 0;
}