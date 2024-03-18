/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_boot.c
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
 use crate::include::*; // I cannot tell how these two lines are different
 use crate::stm32_autoleds::board_autoled_initialize;
 /****************************************************************************
  * Public Functions
  ****************************************************************************/
 
 /****************************************************************************
  * Name: stm32_boardinitialize
  *
  * Description:
  *   All STM32 architectures must provide the following entry point.
  *   This entry point is called early in the initialization -- after all
  *   memory has been configured and mapped but before any devices have been
  *   initialized.
  *
  ****************************************************************************/
 // https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
 // #ifdef does not exist, therefor we will use cfg [ conditional configuration check ]
  fn stm32_boardinitialize()
 {
    if cfg!(CONFIG_ARCH_LEDS){
       /* Configure on-board LEDs if LED support has been selected. */
       board_autoled_initialize();
      }
    // CONFIG_STM32F7_HOST is missing from files
    if  cfg!(CONFIG_STM32F7_OTGFS) || cfg!(CONFIG_STM32F7_HOST) 
    {
       stm32_usbinitialize();
      }
    
    if cfg!(CONFIG_SPI){
       /* Configure SPI chip selects */
       stm32_spidev_initialize();
      }
    
}

/****************************************************************************
 #[cfg(CONFIG_SPI)]
 * Name: board_late_initialize
 *
 * Description:
  *   If CONFIG_BOARD_LATE_INITIALIZE is selected, then an additional
  *   initialization call will be performed in the boot-up sequence to a
  *   function called board_late_initialize().  board_late_initialize() will
  *   be called immediately after up_initialize() is called and just before
  *   the initial application is started. This additional initialization
  *   phase may be used, for example, to initialize board-specific device
  *   drivers.
  *
  ****************************************************************************/
  
  #[cfg(CONFIG_BOARD_LATE_INITIALIZE)]
 fn board_late_initialize()
 {
   /* Perform board-specific initialization */
 
   stm32_bringup();
 }
 