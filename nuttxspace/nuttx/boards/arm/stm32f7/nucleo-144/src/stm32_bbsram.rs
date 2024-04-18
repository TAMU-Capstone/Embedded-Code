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
use crate::bindings::*;
use cty;

#[cfg(CONFIG_STM32F7_BBSRAM)]
{
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
    pub struct stack
    {
        sp: u32;
        top: u32;
        size: u32;
    }
    
    #[allow(non_camel_case_types)]
    pub struct stacks
    {
        user: cty::stack_t;
        if #[cfg(CONFIG_ARCH_INTERRUPTSTACK)] > 3
        {
            interrupt: cty::stack_t;
        }
    }
    
    /* Flags to identify what is in the dump */
    enum fault_flags
    {
        const REGS_PRESENT: u32 = 0x01,
        const USERSTACK_PRESENT: u32 = 0x02,
        const INTSTACK_PRESENT: u32 = 0x04,
        const INVALID_USERSTACK_PTR: u32 = 0x20,
        const INVALID_INTSTACK_PTR: u32: = 0x40,
    }
    
    #[allow(non_camel_case_types)]
    pub struct info
    {
        flags: cty::fault_flags;
        current_regs: usize;
        lineno: i32;
        pid: i32;
        regs: [u32; XCPTCONTEXT_REGS];
        stacks: cty::stack_t;
    
        if #[cfg(CONFIG_TASK_NAME_SIZE)] > 0
        {
            name: [char, CONFIG_TASK_NAME_SIZE + 1];
        }
        
        filename: [char, MAX_FILE_PATH_LENGTH];
    }
    
    #[allow(non_camel_case_types)]
    pub struct fullcontext
    {
        info: i32;
    
        if #[cfg(CONFIG_ARCH_INTERRUPTSTACK)] > 3
        {
            //stack_word_t is a u32
            istack: [u32; CONFIG_USTACK_SIZE];
        }
        
        ustack: [u32, CONFIG_ISTACK_SIZE];
    }
    
    /****************************************************************************
     * Private Data
     ****************************************************************************/
    
    static gsdata: [u8, STM32F7_BBSRAM_SIZE];
    
    #[no_mangle]
    pub unsafe extern "C" fn hardfault_get_desc(desc: mut* struct bbsramd_s) -> cty::c_int
    {
        let filestruct: struct file;
        let ret: i32;
    
        ret = unsafe{file_open(&filestruct, HARDFAULT_PATH, O_RDONLY)};
    
        if (ret < 0)
        {
          unsafe
          {
            syslog(LOG_INFO.into(), "stm32 bbsram: Failed to open Fault Log file [%s] ""(%d)\n".as_ptr() as *const u8, HARDFAULT_PATH, ret);
          }
        }
        else
        {
            unsafe
            {
                ret = file_ioctl(&filestruct, STM32F7_BBSRAM_GETDESC_IOCTL, (unsigned long)((uintptr_t)desc));
                file_close(&filestruct);
    
                if (ret < 0)
                {
                    syslog(LOG_INFO.into(), "stm32 bbsram: Failed to get Fault Log ""descriptor (%d)\n".as_ptr() as *const u8, ret);
                }
            }
        }
    
        return ret;
    }
    
    #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
    {
        #[no_mangle]
        static pub unsafe extern "C" fn copy_reverse(stack_word_t *dest, stack_word_t *src, int size)
        {
            while size--
            {
                *dest++ = *src--;
            }
        }
    }
    
    #[no_mangle]
    pub unsafe extern "C" fn stm32_bbsram_int() -> cty::c_int
    {
        let mut filesizes: [i32, CONFIG_STM32F7_BBSRAM_FILES + 1] = BSRAM_FILE_SIZES;
        buf: [char, HEADER_TIME_FMT_LEN + 1];
        desc: struct bbsramd_s;
        rv: i32;
        state: i32;
        tt: struct tm;
        //time_t is an i64 in rust
        time_sec: i64;
    
        /* Using Battery Backed Up SRAM */
        unsfae{stm32_bbsraminitialize(BBSRAM_PATH, filesizes)};
    
        #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
        {
            let mut rv: i32 = unsafe{hardfault_get_desc(&desc)};
            if rv == OK
            {
                unsafe
                {
                    syslog(LOG_EMERG.into(), "There is a hard fault logged.\n".as_ptr() as *const u8);
                    let mut state: i32 = (desc.lastwrite.tv_sec || desc.lastwrite.tv_nsec) ?  OK : 1;

                    syslog(LOG_INFO.int(), "Fault Log info File No %d Length %d flags:0x%02x ""state:%d\n".as_ptr() as *const u8, (unsigned int)desc.fileno, (unsigned int)desc.len, (unsigned int)desc.flags, state);

                    if (state == OK)
                    {
                        //time_t is an i64 in rust
                        let mut time_sec: i64 = desc.lastwrite.tv_sec + (desc.lastwrite.tv_nsec / 1e9);
                        gmtime_r(&time_sec, &tt);
                        strftime(buf, HEADER_TIME_FMT_LEN, HEADER_TIME_FMT, &tt);

                        syslog(LOG_INFO.into(), "Fault Logged on %s - Valid\n".as_ptr() as *const u8, buf);
                    }

                    rv = nx_unlink(HARDFAULT_PATH);
                    if (rv < 0)
                    {
                        syslog(LOG_INFO.into(), "stm32 bbsram: Failed to unlink Fault Log file"" [%s] (%d)\n".as_ptr() as *const u8, HARDFAULT_PATH, rv);
                    }
                }
            }
        }

        return rv;
    }

    #[cfg(CONFIG_STM32F7_SAVE_CRASHDUMP)]
    {
        pub unsafe extern "C" fn board_crashdump(uintptr_t sp, struct tcb_s *tcb, const char *filename, int lineno, const char *msg, void *regs)
        {
            
        }
    }
}
