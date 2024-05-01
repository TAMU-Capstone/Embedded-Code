/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_can.c
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
#![cfg(CONFIG_CAN)] 
/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*;
use crate::err;
use cty;

 /****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/
#[cfg(CONFIG_STM32F7_CAN1)]
const CAN_PORT: u8 = 1;

#[cfg(not(CONFIG_STM32F7_CAN1))]
const CAN_PORT: u8 = 2;


/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_can_setup
 *
 * Description:
 *  Initialize CAN and register the CAN device
 *
 ****************************************************************************/
#[no_mangle]
 pub extern "C" fn stm32_can_setup() -> cty::c_int {
   
    #[cfg(CONFIG_STM32F7_CAN1)] {
        let mut can = can_dev_s {
            cd_crefs: 0,
            cd_npendrtr: 0,
        };
        let ret: cty::c_int = 0;   
        
        /* Call stm32f7can_initialize() to get an instance of the CAN interface */
        can = stm32_caninitialize(CAN_PORT);
        
        // took out check if can is null ptr since it is initialized to 0s when declared above 
        // prob need to fix this because it assumes that the 2 values in the struct would not be 0 
        if can.cd_crefs == 0 && can.cd_npendrtr == 0 {
            err!("ERROR: Failed to get CAN interface\n");
            return -(ENODEV as i32) // original return was -ENODEV
        }

        /* Register the CAN driver at "/dev/can0" */
        ret = can_register("/dev/can0", can);
        if ret < 0 {
            err!("ERROR: can_register failed: %d\n", ret);
            ret
        }

        OK //
    } // end CONFIG_STM32F7_CAN1

    #[cfg(CONFIG_STM32F7_CAN2)] {
        
        let mut can = can_dev_s {
            cd_crefs: 0,
            cd_npendrtr: 0,
        };
        let ret: cty::c_int = 0;  

        /* Call stm32f7can_initialize() to get an instance of the CAN interface */
        can = stm32_caninitialize(CAN_PORT);

        // took out check if can is null ptr since it is initialized to 0s when declared above 
        // prob need to fix this because it assumes that the 2 values in the struct would not be 0 
        if can.cd_crefs == 0 && can.cd_npendrtr == 0 {
            err!("ERROR: Failed to get CAN interface\n");
            return -(ENODEV as i32) // original return was -ENODEV
        }

         /* Register the CAN driver at "/dev/can0" */
         ret = can_register("/dev/can1", can);
         if ret < 0 {
             err!("ERROR: can_register failed: %d\n", ret);
             ret
         }

        OK 
    } // end CONFIG_STM32F7_CAN2


    OK //
 } // end stm32_can_setup()

