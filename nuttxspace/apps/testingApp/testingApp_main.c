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
#include <stdbool.h> //why do I have to do this??????????????????????????????????????/
#include <unistd.h>


/****************************************************************************
 * Function Definitions
 ****************************************************************************/

extern int board_userled_initialize(void);
extern void board_userled(int led, _Bool ledon); //Necessary??????????????????????????///


/****************************************************************************
 * Test Functions
 ****************************************************************************/

void userleds_test(void) {
  int ledset = 0;

  // configure leds
  int NUM_LEDS = board_userled_initialize();

  // Turn on all three leds
  for (int i = 0; i < NUM_LEDS; i++) {
    board_userled(i, true);
  }

  printf("Test 1 Userleds:\n");
  printf("All three Userleds should be on for 10 seconds.\n");

  sleep(10);

  for (int i = 0; i < NUM_LEDS; i++) {
    board_userled(i, false);
  }

}

/****************************************************************************
 * testingApp_main
 ****************************************************************************/
int main(int argc, FAR char *argv[]) {
  userleds_test();

  return 0;
}
