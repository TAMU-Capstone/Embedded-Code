/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_pwm.c
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
use crate::bindings::*;
//use crate::lazy_static::*;
/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_pwm_setup
 *
 * Description:
 *   Initialize PWM and register the PWM device.
 *
 ****************************************************************************/
//DEFINE config first -- encompasses all


#[cfg(CONFIG_PWM)]
#[no_mangle]
pub extern "C" fn stm32_pwm_setup() -> cty::c_int
{
    use core::mem::MaybeUninit;
    static mut INITIALIZED: bool = false;

    if unsafe { !INITIALIZED } {
        let mut pwm: MaybeUninit<*mut pwm_lowerhalf_s> = MaybeUninit::uninit();

        #[cfg(CONFIG_STM32F7_TIM1_PWM)]
        unsafe {
            pwm.write(stm32_pwminitialize(1));
            if pwm.assume_init().is_null() {
                return -(ENODEV as i32);
            }
            match pwm_register(b"/dev/pwm0\0" as *const u8, pwm.assume_init()) {
                OK => (),
                ret => return ret
            }
        }
        
        #[cfg(CONFIG_STM32F7_TIM2_PWM)]
        unsafe {
            pwm.write(stm32_pwminitialize(2));
            if pwm.assume_init().is_null() {
                return -(ENODEV as i32);
            }
            match pwm_register(b"/dev/pwm1\0" as *const u8, pwm.assume_init()) {
                OK => (),
                ret => return ret
            }
        }
        
        #[cfg(CONFIG_STM32F7_TIM3_PWM)]
        unsafe {
            pwm.write(stm32_pwminitialize(3));
            if pwm.assume_init().is_null() {
                return -(ENODEV as i32);
            }
            match pwm_register(b"/dev/pwm2\0" as *const u8, pwm.assume_init()) {
                OK => (),
                ret => return ret
            }
        }
        
        #[cfg(CONFIG_STM32F7_TIM4_PWM)]
        unsafe {
            pwm.write(stm32_pwminitialize(4));
            if pwm.assume_init().is_null() {
                return -(ENODEV as i32);
            }
            match pwm_register(b"/dev/pwm3\0" as *const u8, pwm.assume_init()) {
                OK => (),
                ret => return ret
            }
        }

        unsafe { INITIALIZED = true }
    }
    return OK;
}



#[cfg(not(CONFIG_PWM))]
#[no_mangle]
pub extern "C" fn stm32_pwm_setup() -> cty::c_int
{
    -(ENODEV as i32)
}
