/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_sdio.c
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
 use crate::include::*;
 use lazy_static::lazy_static;

//DEFINE config first -- encompasses all
cfg_if::cfg_if! 
{ 
    if #[cfg(CONFIG_MMCSD)]
    {
        //if macro defined, have constant
        if #[cfg(GPIO_SDMMC1_NCD)]
        {
            const HAVE_NCD: i32 = 1;
        }

        //define struct statically
        //https://users.rust-lang.org/t/static-struct-with-a-string-inside-a-module-for-singleton-pattern/37475
        //https://github.com/rust-lang-nursery/lazy-static.rs
        lazy_static!
        {
            static ref g_sdio_dev = &mut sdio_dev_s;
        }

        //if constant is defined
        if #[cfg(HAVE_NCD)]
        {
            lazy_static!
            {
                static bool g_sd_inserted;
            }
        }


    }
}