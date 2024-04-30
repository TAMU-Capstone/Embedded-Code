/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_cansock.c
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
 #![cfg(any(CONFIG_STM32F7_CAN1, CONFIG_STM32F7_CAN2))]

/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*; 
use crate::{err, info};
use core::ptr;
use crate::cty;
 
/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/

/* Configuration ************************************************************/
#[cfg(not(any(CONFIG_STM32F7_CAN1, CONFIG_STM32F7_CAN2)))]
compile_error!("No CAN is enabled. Please enable at least one CAN device");

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_cansock_setup
 *
 * Description:
 *  Initialize CAN socket interface
 *
 ****************************************************************************/
 
 pub extern "C" fn stm32_cansock_setup() -> cty::c_int {
    let mut _ret: cty::c_int = OK;

    #[cfg(CONFIG_STM32F7_CAN1)] {
        /* Call stm32_caninitialize() to get an instance of the CAN interface */
        _ret = stm32_cansockinitialize(1);

        /*
        After calling canerr, the code uses the goto statement to jump to a label named errout. The goto statement is used for unconditional jumps to a labeled statement within the same function.
        */
        if (_ret < 0) {
            err!("ERROR: Failed to get CAN interface %d\n", _ret);
        }   
    }

    #[cfg(CONFIG_STM32F7_CAN2)] {
        /* Call stm32_caninitialize() to get an instance of the CAN interface */
        _ret = stm32_cansockinitialize(2);

        /*
        After calling canerr, the code uses the goto statement to jump to a label named errout. The goto statement is used for unconditional jumps to a labeled statement within the same function.
        */
        if (_ret < 0) {
            err!("ERROR: Failed to get CAN interface %d\n", _ret);
        }   
    }
    // errout was removed 
    _ret //CHANGE
 }
 