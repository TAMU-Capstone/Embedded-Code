/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_autoleds.c
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


 
 /****************************************************************************
  * Private Data
  ****************************************************************************/
 
 /* Indexed by BOARD_LED_<color> */
 static g_ledmap: [u32; BOARD_NLEDS] = [GPIO_LED_GREEN, GPIO_LED_BLUE, GPIO_LED_RED];

 static mut g_initialized: bool = false;
 /****************************************************************************
  * Private Functions
  ****************************************************************************/
 
  extern "C" {
    fn stm32_gpiowrite(pin: u32, state: bool);
  }

  fn phy_set_led(led: usize, state: bool) {
    stm32_gpiowrite(g_ledmap[led], state);
  }
 
 /****************************************************************************
  * Public Functions
  ****************************************************************************/
 
 /****************************************************************************
  * Name: board_autoled_initialize
  ****************************************************************************/
  fn board_autoled_initialize() {
    for &pin in g_ledmap.iter() {
        stm32_configgpio(pin);
    }
  }

 
 /****************************************************************************
  * Name: board_autoled_on
  ****************************************************************************/
 
 
 
 /****************************************************************************
  * Name: board_autoled_off
  ****************************************************************************/
 
 
 
 