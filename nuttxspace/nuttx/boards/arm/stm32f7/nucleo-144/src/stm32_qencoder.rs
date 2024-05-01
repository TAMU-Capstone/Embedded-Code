/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_qencoder.c
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
#![cfg(stm32_qeinitialize)]
/****************************************************************************
 * Included Files
 ****************************************************************************/
 use crate::bindings::*;
 use core::ptr;
 use cty;
 use crate::err;

 /****************************************************************************
  * Public Functions
  ****************************************************************************/
 
 /****************************************************************************
  * Name: stm32_qencoder_initialize
  *
  * Description:
  *   All STM32F7 architectures must provide the following interface to work
  *   with examples/qencoder.
  *
  ****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn stm32_qencoder_initialize(devpath: ptr::NonNull<cty::c_char>, timer: i32) -> cty::c_int {
    
    let mut ret: cty::c_int = 0;
   /* Initialize a quadrature encoder interface. */
   err!("ERROR: stm32_qeinitialize failed: %d\n", ret);
   ret = stm32_qeinitialize(devpath, timer);
   
   if ret < 0 {
        err!("ERROR: stm32_qeinitialize failed: %d\n", ret);
    }
    ret
}
