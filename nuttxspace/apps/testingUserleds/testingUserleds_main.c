/****************************************************************************
 * apps/testingLed/testingLed_main.c
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
#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>
#include <stddef.h>
#include <sys/param.h>

#ifndef CONFIG_ARCH_LEDS
/****************************************************************************
 * Function Definitions
 ****************************************************************************/
uint32_t board_userled_initialize(void);
void board_userled(int led, bool ledon);
/****************************************************************************
 * Test Function
 ****************************************************************************/

void userleds_test(void) {
  // configure leds
  uint32_t NUM_LEDS = board_userled_initialize();

  printf("Testing Userleds:\n");
  printf("All three Userleds should be on for 5 seconds. (Blue turns on only momentarily)\n");

  // Turn on all three leds
  for (int i = 0; i < NUM_LEDS; i++) {
    board_userled(i, true);
  }

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
#endif
#ifdef CONFIG_ARCH_LEDS
int main(int argc, FAR char *argv[])
{
  printf("User leds is defined therefore autoleds cannot be\n");
}
#endif