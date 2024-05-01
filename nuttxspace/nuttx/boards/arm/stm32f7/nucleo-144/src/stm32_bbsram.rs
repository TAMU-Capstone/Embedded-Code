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

use core::default;

use crate::bindings::*;
use crate::{info, err};
use cty;
use core::mem::size_of;

/****************************************************************************
 * Pre-processor Definitions
****************************************************************************/

/* Configuration ************************************************************/
const HARDFAULT_FILENO: usize = 3;
const HARDFAULT_PATH: &'static str = concat!("/fs/bbr", 3, '\0');     // Can't do concat!(HARDFAULT_PATH, HARDFAULT_FILENO) yet ://
const HARDFAULT_REBOOT_FILENO: usize  = 0;
const HARDFAULT_REBOOT_PATH: &'static str = concat!("/fs/bbr", 0, '\0');

const BBSRAM_SIZE_FN0: i32 = size_of::<i32>() as i32;
const BBSRAM_SIZE_FN1: i32 = 384;
const BBSRAM_SIZE_FN2: i32 = 384;
const BBSRAM_SIZE_FN3: i32 = -1;

const BBSRAM_HEADER_SIZE: u32 = 20;
const BBSRAM_USED: u32 = (4 * BBSRAM_HEADER_SIZE) + (BBSRAM_SIZE_FN0 + BBSRAM_SIZE_FN1 + BBSRAM_SIZE_FN2) as u32;
const BBSRAM_REMAINING: u32 = STM32F7_BBSRAM_SIZE as u32 - BBSRAM_USED;
const BBSRAM_NUMBER_STACKS: u32 = if CONFIG_ARCH_INTERRUPTSTACK <= 3 { 1 } else { 2 };
const BBSRAM_FIXED_ELEMENTS_SIZE: u32 = size_of::<info_t>() as u32;
const BBSRAM_LEFTOVER: u32 = BBSRAM_REMAINING - BBSRAM_FIXED_ELEMENTS_SIZE;
const CONFIG_ISTACK_SIZE: u32 = BBSRAM_LEFTOVER / BBSRAM_NUMBER_STACKS / size_of::<stack_word_t>() as u32;
const CONFIG_USTACK_SIZE: u32 = BBSRAM_LEFTOVER / BBSRAM_NUMBER_STACKS / size_of::<stack_word_t>() as u32;

const BBSRAM_PATH: &'static str = "/fs/bbr\0";

const BSRAM_FILE_SIZES: [i32; 5] = [
    BBSRAM_SIZE_FN0,
    BBSRAM_SIZE_FN1,
    BBSRAM_SIZE_FN2,
    BBSRAM_SIZE_FN3,
    0,
];

const MAX_FILE_PATH_LENGTH: usize = 40;

const HEADER_TIME_FMT: &'static str = "%Y-%m-%d-%H:%M:%S\0";
const HEADER_TIME_FMT_NUM: usize = 2 + 0 + 0 + 0 + 0 + 0;
const HEADER_TIME_FMT_LEN: usize = (HEADER_TIME_FMT.len() - 1) + HEADER_TIME_FMT_NUM;

/****************************************************************************
 * Private Data
****************************************************************************/
#[allow(non_camel_case_types)]
type stack_word_t = u32;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone)]
pub struct stack_t {
    sp: u32,
    top: u32,
    size: u32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone)]
pub struct stacks_t {
    user: stack_t,
    // #[cfg(CONFIG_ARCH_INTERRUPTSTACK)]
    interrupt: stack_t
}

/* Flags to identify what is in the dump */
#[repr(u8)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
enum fault_flags_t
{
    #[default]
    REGS_PRESENT = 0x01,
    USERSTACK_PRESENT = 0x02,
    INTSTACK_PRESENT = 0x04,
    INVALID_USERSTACK_PTR = 0x20,
    INVALID_INTSTACK_PTR = 0x40,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct info_t {
    flags: fault_flags_t,
    current_regs: usize,
    lineno: i32,
    pid: i32,
    regs: [u32; XCPTCONTEXT_REGS as usize],
    stacks: stacks_t,
    #[cfg(CONFIG_TASK_NAME_SIZE)]
    name: [u8; CONFIG_TASK_NAME_SIZE as usize + 1],
    filename: [u8; MAX_FILE_PATH_LENGTH],
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct fullcontext_t {
    info: info_t,
    // #[cfg(CONFIG_ARCH_INTERRUPTSTACK)]
    istack: [usize; CONFIG_USTACK_SIZE as usize],
    ustack: [usize; CONFIG_ISTACK_SIZE as usize],
}

/****************************************************************************
 * Private Data
****************************************************************************/

static mut G_SDATA: [u8; STM32F7_BBSRAM_SIZE as usize] = [0; STM32F7_BBSRAM_SIZE as usize];

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Name: hardfault_get_desc
 ****************************************************************************/

fn hardfault_get_desc(desc: *mut bbsramd_s) -> Result<(), i32> {
    let filestruct: *mut file;

    let ret = unsafe{ file_open(filestruct, HARDFAULT_PATH.as_ptr(), O_RDONLY as i32) };

    if ret < 0 {
        info!("stm32 bbsram: Failed to open Fault Log file [%s] (%d)\n", HARDFAULT_PATH.as_ptr(), ret);
        return Err(ret);
    }
    else {
        unsafe {
            ret = file_ioctl(filestruct, STM32F7_BBSRAM_GETDESC_IOCTL, desc);
            file_close(filestruct);
        }

        if ret < 0 {
            info!("stm32 bbsram: Failed to get Fault Log descriptor (%d)\n", ret);
            return Err(ret);
        }
    }

    Ok(())
}

// #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
fn copy_reverse(dest: *mut stack_word_t, src: *mut stack_word_t, size: i32) {
    for i in 0..size as isize{
        unsafe {
            *dest.offset(i) = *src.offset(-i);
        }
    }
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

#[no_mangle]
pub extern "C" fn stm32_bbsram_int() -> cty::c_int
{
    let mut filesizes: [i32; CONFIG_STM32F7_BBSRAM_FILES as usize + 1] = BSRAM_FILE_SIZES;
    let mut buf: [u8; HEADER_TIME_FMT_LEN + 1];
    let mut desc: bbsramd_s;
    let mut state: i32;
    let tt: tm;
    //time_t is an i64 in rust

    /* Using Battery Backed Up SRAM */
    unsafe{ 

        stm32_bbsraminitialize(BBSRAM_PATH.as_mut_ptr(), filesizes.as_mut_ptr() as *mut i32)
    };

    #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
    match hardfault_get_desc(&mut desc) {
        Err(e) => return e,
        Ok(_) => {
            unsafe {
                syslog(LOG_EMERG.into(), "There is a hard fault logged.\n\0".as_ptr());
            }
            state = !(desc.lastwrite.tv_sec != 0 || desc.lastwrite.tv_nsec != 0) as i32;

            info!("Fault Log info File No %d Length %d flags:0x%02x state:%d\n", desc.fileno,  desc.len, desc.flags, state);

            if state == OK {
                let time_sec: time_t = desc.lastwrite.tv_sec + (desc.lastwrite.tv_nsec / 1e9);
                unsafe{
                    gmtime_r(&time_sec, &mut tt);
                    strftime(buf.as_mut_ptr(), HEADER_TIME_FMT_LEN, HEADER_TIME_FMT, &tt);
                }
                info!("Fault Logged on %s - Valid\n", buf);
            }

            match unsafe { nx_unlink(HARDFAULT_PATH.as_ptr()) } {
                OK => OK,
                res => {
                    info!("stm32 bbsram: Failed to unlink Fault Log file [%s] (%d)\n", HARDFAULT_PATH, res);
                    res
                }
            }
        }
    }

    #[cfg(not(CONFIG_STM32F7_SAVE_CRASHDUMP))]
    OK
}

// #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
#[no_mangle]
pub extern "C" fn board_crashdump(
    sp: usize, tcb: *mut tcb_s,
    filename: *const cty::c_char, lineno: i32, 
    msg: *const cty::c_char, regs: *mut cty::c_void
) {

    let pdump = unsafe { G_SDATA.as_mut_ptr() } as *mut fullcontext_t;
    let mut rv: i32;


    unsafe { enter_critical_section() };

    unsafe { memset(pdump as *mut cty::c_void, 0, size_of::<fullcontext_t>() as u32) };
    
    unsafe { (*pdump).info.lineno = lineno };
    
    
    if !filename.is_null()
    {
        let mut offset: i32 = 0;
        let len = unsafe { strlen(filename) } + 1;
        
        if len > size_of::<[u8; MAX_FILE_PATH_LENGTH]>() as u32 {
            offset = (len - size_of::<[u8; MAX_FILE_PATH_LENGTH]>() as u32) as i32;
        }
        
        unsafe{
            strlcpy((*pdump).info.filename.as_mut_ptr(), filename.offset(offset as isize), size_of::<[u8; MAX_FILE_PATH_LENGTH]>())
        };
    }
    
    unsafe { (*pdump).info.current_regs = g_current_regs[up_cpu_index()] as usize };
    
    #[cfg(CONFIG_TASK_NAME_SIZE)]
    unsafe{ strlcpy((*pdump).info.name.as_mut_ptr(), (*tcb).name, size_of::<[u8; CONFIG_TASK_NAME_SIZE as usize + 1]>()) };
    
    unsafe { (*pdump).info.pid = (*tcb).pid };

    // Good luck bro, start here
    

    if CURRENT_REGS != 0 {
        unsafe{
            (*pdump).info.stacks.interrupt.sp = sp as u32;
            (*pdump).info.flags = (*pdump).info.flags | fault_flags_t::REGS_PRESENT | fault_flags_t::USERSTACK_PRESENT | fault_flags_t::INTSTACK_PRESENT;
            memcpy((*pdump).info.regs, CURRENT_REGS, size_of<((*pdump).info.regs));
            (*pdump).info.stacks.user.sp = (*pdump).info.regs[REG_R13];
        }
    }
    else {
        (*pdump).info.flags |= fault_flags_t::USERSTACK_PRESENT as u8;
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
        (*pdump).info.flags != fault_flags_t::INVALID_INTSTACK_PTR as u8;
        }
    }
    if (*pdump).info.flags & fault_flags_t::USERSTACK_PRESENT as u8 != 0
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

    let rv = stm32_bbsram_savepanic(HARDFAULT_FILENO as i32, pdump as *mut u8, size_of<(fullcontext_t));

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