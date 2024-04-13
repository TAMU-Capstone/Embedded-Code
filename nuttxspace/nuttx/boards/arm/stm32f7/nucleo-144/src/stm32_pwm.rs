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

#[no_mangle]
pub extern "C" fn stm32_pwm_setup() -> cty::c_int
{
    #[cfg(HAVE_PWM)]
    {
        let mut initialized: bool = false;
        let mut pwm: &mut pwm_lowerhalf_s;

        let mut ret: i32;

        const ENODEV: i32 = -1;

        if initialized == true
        {
            #[cfg(CONFIG_STM32F7_TIM1_PWM)]
            {
                let pwm = unsafe{ stm32_pwminitialize(1) };

                if !pwm
                {
                    unsafe{
                        aerr("ERROR: Failed to get the STM32F7 PWM lower half\n");
                    }
                    return ENODEV.into();
                }

                ret = unsafe{ pwm_register("/dev/pwm0".as_ptr(), pwm) };

                if ret < 0
                {
                    aerr("ERROR: pwm_register failed: %d\n".as_ptr(), ret);
                    return ret;
                }
            }

            #[cfg(CONFIG_STM32F7_TIM2_PWM)]
            {
                let pwm = unsafe{ stm32_pwminitialize(2) };
                if !pwm
                {
                    unsafe{
                        aerr("ERROR: Failed to get the STM32F7 PWM lower half\n".as_ptr());
                    }
                    return ENODEV.into();
                }

                ret = unsafe{ pwm_register("/dev/pwm1".as_ptr(), pwm) };
                if ret < 0
                {
                    unsafe{
                        aerr("ERROR: pwm_register failed: %d\n".as_ptr(), ret);
                    }
                    return ret;
                }
            }

            #[cfg(CONFIG_STM32F7_TIM3_PWM)]
            {
                let pwm = unsafe{ stm32_pwminitialize(3) };
                if !pwm
                {
                    unsafe{
                        aerr("ERROR: Failed to get the STM32F7 PWM lower half\n".as_ptr());
                    }
                    return ENODEV.into();
                }

                ret = unsafe{ pwm_register("/dev/pwm2".as_ptr(), pwm) };
                if ret < 0
                {
                    unsafe{
                        aerr("ERROR: pwm_register failed: %d\n".as_ptr(), ret);
                    }
                    return ret;
                }
            }

            #[cfg(CONFIG_STM32F7_TIM4_PWM)]
            {
                let pwm = unsafe{ stm32_pwminitialize(4) };
                if !pwm
                {
                    unsafe{
                        aerr("ERROR: Failed to get the STM32F7 PWM lower half\n".as_ptr());
                    }
                    return ENODEV.into();
                }

                ret = unsafe{ pwm_register("/dev/pwm3".as_ptr(), pwm) };
                if ret < 0
                {
                    unsafe{
                        aerr("ERROR: pwm_register failed: %d\n", ret);
                    }
                    return ret;
                }
            }
            initialized = true;
        }
        return OK;
    }
    #[cfg(not(HAVE_PWM))]
    {
        return ENODEV.into();
    }
}
