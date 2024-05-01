/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_romfs_initialize.c
 * This file provides contents of an optional ROMFS volume, mounted at boot.
 *
 *   Copyright (C) 2017 Tomasz Wozniak. All rights reserved.
 *   Author: Tomasz Wozniak <t.wozniak@samsung.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 * 3. Neither the name NuttX nor the names of its contributors may be
 *    used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 * OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED
 * AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 * ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 ****************************************************************************/
#![cfg(CONFIG_STM32_ROMFS)]

/****************************************************************************
 * Included Files
 ****************************************************************************/
 use crate::bindings::*; 
 use heapless::String;
 use core::fmt::Write;
 use core::ptr::null_mut;
 use cty;

 /* Anything wrapped in these is most likely not used so remove these blocks 
  Constants/functions I am missing for romfs_initilaize 
  CONFIG_STM32_ROMFS_DEV_MINOR
  CONFIG_STM32_ROMFS
  CONFIG_STM32_ROMFS_IMAGEFILE
  CONFIG_STM32_ROMFS_DEV_MINOR
  CONFIG_STM32_ROMFS_MOUNTPOINT
  MOUNT_DEVNAME
  romdisk_register
*/

/****************************************************************************
 * Originally in header file
 ****************************************************************************/
const ROMFS_SECTOR_SIZE: usize = 64;

/****************************************************************************
 * Pre-processor Definitions
 ****************************************************************************/
 
#[cfg(not(CONFIG_STM32_ROMFS))]
compile_error!("CONFIG_STM32_ROMFS must be defined");

#[cfg(not(CONFIG_STM32_ROMFS_IMAGEFILE))]
compile_error!("CONFIG_STM32_ROMFS_IMAGEFILE must be defineds");

#[cfg(not(CONFIG_STM32_ROMFS_DEV_MINOR))]
compile_error!("CONFIG_STM32_ROMFS_DEV_MINOR must be defined");

#[cfg(not(CONFIG_STM32_ROMFS_MOUNTPOINT))]
compile_error!("CONFIG_STM32_ROMFS_MOUNTPOINT must be defined");

fn nsectors(size: usize) -> usize {
  (size + ROMFS_SECTOR_SIZE - 1) / ROMFS_SECTOR_SIZE
}

// #define STR2(m)  #m
// #define STR(m) STR2(m)
// #define MKMOUNT_DEVNAME(m) "/dev/ram" STR(m)

// THIS ASSUMES THAT "m" is an int could not get it to work where m can be of any type
// for some reason cannot use this function goes back and forth between saying use const dont use const 
fn mkmount(m: u32) -> String<32> {
  let mut result = String::new();
  result.push_str("/dev/ram").unwrap();
  write!(result, "{}", m).unwrap();
  result
}

/****************************************************************************
 * Private Data
 ****************************************************************************/
/*
This code is embedding the contents of a ROM file system image directly into the executable's read-only data section
 The romfs_data_begin and romfs_data_end symbols mark the start and end of the data
 C code used asm Rust does not have built-in support for inline assembly

 The include_bytes! macro is a built-in Rust feature that takes care of embedding the binary data from  file into the executable's 
 read-only data section aligning the data as needed, and making it available as a byte slice
*/

static ROMFS_DATA: &[u8] = include_bytes!("CONFIG_STM32_ROMFS_IMAGEFILE.txt"); // FIX THIS NEED THIS CONSTANT BEFORE I CAN SEND AS A STRING using a temp file for now
// we may not need these they mark the beginning and end of the data
//static romfs_data_begin: &[u8] = ROMFS_DATA;
//static romfs_data_end: &[u8] = &ROMFS_DATA[ROMFS_DATA.len()..];

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: stm32_romfs_initialize
 *
 * Description:
 *   Registers the aboveincluded binary file as block device.
 *   Then mounts the block device as ROMFS filesystems.
 *
 * Returned Value:
 *   Zero (OK) on success, a negated errno value on error.
 *
 * Assumptions/Limitations:
 *   Memory addresses [romfs_data_begin .. romfs_data_end) should contain
 *   ROMFS volume data, as included in the assembly snippet above (l. 84).
 *
 ****************************************************************************/
 

// start here
#[no_mangle]
 pub extern "C" fn stm32_romfs_initialize() -> cty::c_int {

  let romfs_data_len: usize = romfs;
  let ret: i32; 

  /* Create a ROM disk for the /etc filesystem */
  // registering a read-only memory disk and mounting it as a ROMFS filesystem.
  let ret: i32 = unsafe {
    romdisk_register(
        CONFIG_STM32_ROMFS_DEV_MINOR,
        ROMFS_DATA.as_ptr(),
        nsectors(romfs_data_len),
        ROMFS_SECTOR_SIZE,
    )
  };

  if ret < 0 {
      return Err(ret);
  }

  /* Mount the file system */
  // finfo("Mounting ROMFS filesystem at target=%s with source=%s\n", CONFIG_STM32_ROMFS_MOUNTPOINT, MOUNT_DEVNAME);
  let ret = unsafe {
    nx_mount(
        MOUNT_DEVNAME.as_ptr() as *const i8,
        CONFIG_STM32_ROMFS_MOUNTPOINT.as_ptr() as *const i8,
        "romfs\0".as_ptr() as *const u8,
        MS_RDONLY as u32,
        null_mut(),
    )
  };
  
  if ret < 0 {
    return Err(ret);
  }

  Ok(())
 }