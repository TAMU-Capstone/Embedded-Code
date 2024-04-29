/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_bbsram.c
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
//  #![cfg(CONFIG_STM32F7_BBSRAM)]

use crate::bindings::*;
use crate::{info, err};
use core::sizeof;
use core::mem::zeroed;
use cty;

/****************************************************************************
 * Pre-processor Definitions
****************************************************************************/

/* Configuration ************************************************************/



/* The following guides in the amount of the user and interrupt stack
* data we can save. The amount of storage left will dictate the actual
* number of entries of the user stack data saved. If it is too big
* It will be truncated by the call to stm32_bbsram_savepanic
*/

/* The path to the Battery Backed up SRAM */

/* The sizes of the files to create (-1) use rest of BBSRAM memory */

/* For Assert keep this much of the file name */

/****************************************************************************
 * Private Data
****************************************************************************/
#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy)]
pub struct stack {
    sp: u32,
    top: u32,
    size: u32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct stacks {
    user: stack,
    // #[cfg(CONFIG_ARCH_INTERRUPTSTACK)]
    interrupt: stack
}

/* Flags to identify what is in the dump */
#[repr(u8)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

enum fault_flags
{
    REGS_PRESENT = 0x01,
    USERSTACK_PRESENT = 0x02,
    INTSTACK_PRESENT = 0x04,
    INVALID_USERSTACK_PTR = 0x20,
    INVALID_INTSTACK_PTR = 0x40,
}

#[allow(non_camel_case_types)]
pub struct info_t {
    flags: fault_flags,
    current_regs: usize,
    lineno: i32,
    pid: i32,
    regs: [u32; XCPTCONTEXT_REGS],
    stacks: stack,
    #[cfg(CONFIG_TASK_NAME_SIZE)]
    name: [char; CONFIG_TASK_NAME_SIZE + 1],
    filename: [char; MAX_FILE_PATH_LENGTH],
}

#[allow(non_camel_case_types)]
pub struct fullcontext {
    info: info_t,
    // #[cfg(CONFIG_ARCH_INTERRUPTSTACK > 3)]
    istack: [usize; CONFIG_USTACK_SIZE],
    ustack: [usize; CONFIG_ISTACK_SIZE],
}

type stack_word_t = u32;

/****************************************************************************
 * Private Data
****************************************************************************/

static gsdata: [u8; STM32F7_BBSRAM_SIZE];

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Name: hardfault_get_desc
 ****************************************************************************/

#[no_mangle]
pub extern "C" fn hardfault_get_desc(desc: *mut bbsramd_s) -> cty::c_int {
    let filestruct: file;
    let ret: i32;

    ret = unsafe{ file_open(&filestruct, HARDFAULT_PATH, O_RDONLY) };

    if (ret < 0) {
        unsafe {
            info!("stm32 bbsram: Failed to open Fault Log file [%s] ""(%d)\n", HARDFAULT_PATH, ret);
        }
    }
    else {
        unsafe {
            ret = file_ioctl(&filestruct, STM32F7_BBSRAM_GETDESC_IOCTL, desc);
            file_close(&filestruct);

            if (ret < 0) {
                info!("stm32 bbsram: Failed to get Fault Log ""descriptor (%d)\n", ret);
            }
        }
    }

    return ret;
}

// #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
#[no_mangle]
fn copy_reverse(dest: *mut stack_word_t, src: *mut stack_word_t, mut size: i32) {
    while size != 0 {
        unsafe {
            *dest = *src;
            *dest += 1;
            *src -= 1;
        }
        size -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn stm32_bbsram_int() -> cty::c_int
{
    let mut filesizes: [i32; CONFIG_STM32F7_BBSRAM_FILES + 1] = BSRAM_FILE_SIZES;
    let mut buf: [char; HEADER_TIME_FMT_LEN + 1];
    let mut desc: bbsramd_s = core::mem::zeroed();
    let mut rv: i32;
    let mut state: i32;
    let tt: tm;
    //time_t is an i64 in rust

    /* Using Battery Backed Up SRAM */
    unsafe{ 
        stm32_bbsraminitialize(BBSRAM_PATH.as_ptr() as *const char, filesizes.as_ptr())
    };

    // #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
    {
        rv = unsafe{hardfault_get_desc(&mut desc)};
        if rv == OK
        {
            unsafe
            {
                syslog(LOG_EMERG.into(), "There is a hard fault logged.\n".as_ptr() as *const u8);
                state = if desc.lastwrite.tv_sec != 0 || desc.lastwrite.tv_nsec != 0
                {
                0
                }
                else
                {
                1
                };

                syslog(LOG_INFO.int(), "Fault Log info File No %d Length %d flags:0x%02x ""state:%d\n".as_ptr() as *const u8, desc.fileno, desc.len, desc.flags, state);

                if state == OK
                {
                    //time_t is an i64 in rust
                    let time_sec: i64 = desc.lastwrite.tv_sec + (desc.lastwrite.tv_nsec / 1e9);
                    unsafe{
                        gmtime_r(&time_sec, &tt);
                        strftime(buf, HEADER_TIME_FMT_LEN, HEADER_TIME_FMT, &tt);

                        info!("Fault Logged on %s - Valid\n" as *const u8, buf);
                    }
                }

                rv = nx_unlink(HARDFAULT_PATH.as_ptr() as *const char);
                if rv < 0
                {
                    info!("stm32 bbsram: Failed to unlink Fault Log file"" [%s] (%d)\n" as *const u8, HARDFAULT_PATH, rv);
                }
            }
        }
    }

    return rv;
}

// #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
{
    pub unsafe extern "C" fn board_crashdump(sp: usize, struct tcb_s: *mut tcb, filename: *const char, lineno: i32, msg: const char, regs: *mut void)
    {
        pdump = &mut g_sdata as *mut fullcontext;
        let mut rv: i32;

        unsafe{
            enter_critical_section();

            memset(pdump, 0, sizeof(fullcontext_t));
        }

        lineno: (*pdump).info.lineno;

        if filename
        {
            let mut offset: i32 = 0;
            let filename_str = CStr::from_ptr(filename).to_str().unwrap_or("");
            let mut len: u32 = filename_str.len() + 1;

            if len > sizeof((*pdump).info.filename)
            {
                offset = len - sizeof((*pdump).info.filename);
            }

            unsafe{
                strlcpy(pdump.info.filename, )
            }
        }

        (*pdump).info.current_regs = CURRENT_REGS as usize;

        // #[cfg(CONFIG_TASK_NAME_SIZE) > 0]
        unsafe{strlcpy((*pdump).info.name, (*tcb).name, sizeof(*pdump).info.name)};
    
        (*pdump).info.pid = (*tcb).pid;

        if CURRENT_REGS
        {
        (*pdump).info.stacks.interrupt.sp = sp as u32;
        (*pdump).info.flags |= fault_flags::REGS_PRESENT as u8;
        (*pdump).info.flags |= fault_flags::USERSTACK_PRESENT as u8;
        (*pdump).info.flags |= fault_flags::INTSTACK_PRESENT as u8;
        unsafe{
            memcpy((*pdump).info.regs, CURRENT_REGS, sizeof((*pdump).info.regs));
        }
        (*pdump).info.stacks.user.sp = (*pdump).info.regs[REG_R13];
        }
        else
        {
        (*pdump).info.flags |= fault_flags::USERSTACK_PRESENT as u8;
        (*pdump).info.stacks.user.sp = sp as u32;
        }

        (*pdump).info.stacks.user.top = (*tcb).stack_base_ptr as usize + (*tcb).adj_stack_size as u32;
        (*pdump).info.stacks.user.size = (*tcb).adj_stack_size as u32;

        *[cfg(CONFIG_ARCH_INTERRUPTSTACK > 3)]
        {
        (*pdump).info.stacks.interrupt.top = g_intstacktop;
        (*pdump).info.stacks.interrupt.size = (CONFIG_ARCH_INTERRUPTSTACK & !3) as u32;

        if (*pdump).info.flags & fault_flags_t::INTSTACK_PRESENT as u8 != 0
        {
            let ps: stack_word_t = (*pdump).info.stacks.interrupt.sp as *mut stack_word_t;

            unsafe{
                copy_reverse((*pdump).istack.as_mut_ptr(), &ps[nitems((*pdump).istack) / 2], nitems((*pdump).istack));
            }
        }
            if !((*pdump).info.stacks.interrupt.sp <= (*pdump).info.stacks.interrupt.top && (*pdump).info.stacks.interrupt.sp > (*pdump).info.stacks.interrupt.top - (*pdump).info.stacks.interrupt.size)
            {
            (*pdump).info.flags != fault_flags::INVALID_INTSTACK_PTR as u8;
            }
        }
        if (*pdump).info.flags & fault_flags::USERSTACK_PRESENT as u8 != 0
        {
        let ps = (*pdump).info.stacks.user.sp as *mut stack_word_t;
        unsafe{
            copy_reverse((*pdump).ustack, &ps[nitems((*pdump).ustack) / 2], nitems((*pdump).ustack));
        }
        }

        if !((*pdump).info.stacks.user.sp <= (*pdump).info.stacks.user.top && (*pdump).info.stacks.user.sp > (*pdump).info.stacks.user.top - (*pdump).info.stacks.user.size)
        {
            (*pdump).info.flags |= fault_flags_t::INVALID_USERSTACK_PTR as u8;
        }

        let rv = stm32_bbsram_savepanic(HARDFAULT_FILENO as i32, pdump as *mut u8, sizeof(fullcontext));

        if rv == -EXNIO
        {
        let dead = b"Memory wiped - dump not saved!\0";
        while *dead
        {
            unsafe{
                arm_lowputc(*dead++);
            }
        }
        }
        else if rv == -ENOSPC
        {
        arm_lowputc('!');
        }
    }
}