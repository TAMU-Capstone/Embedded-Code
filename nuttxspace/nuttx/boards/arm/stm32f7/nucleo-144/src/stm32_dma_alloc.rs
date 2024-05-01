/****************************************************************************
 * boards/arm/stm32f7/nucleo-144/src/stm32_dma_alloc.c
 *
 *   Copyright (C) 2016 PX4 Development Team. All rights reserved.
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
 * 3. Neither the name PX4 nor the names of its contributors may be
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
#![cfg(CONFIG_FAT_DMAMEMORY)]

/****************************************************************************
 * Included Files
 ****************************************************************************/
use crate::bindings::*; 
use core::mem::size_of_val;
use core::ptr;
use cty;

/****************************************************************************
* Pre-processor Definitions
****************************************************************************/
#[cfg(not(CONFIG_GRAN))]
compile_error!("microSD DMA support requires CONFIG_GRAN");

// define the size of the memory pool used for DMA allocations 8 blocks of 512 bytes each
const BOARD_DMA_ALLOC_POOL_SIZE: usize = 8 * 512;

/****************************************************************************
* Private Data
****************************************************************************/
static mut DMA_ALLOCATOR: *mut cty::c_void = ptr::null_mut();

/* The DMA heap size constrains the total number of things that can be
* ready to do DMA at a time.
*
* For example, FAT DMA depends on one sector-sized buffer per filesystem
* plus one sector-sized buffer per file.
*
* We use a fundamental alignment / granule size of 64B; this is sufficient
* to guarantee alignment for the largest STM32 DMA burst
* (16 beats x 32bits).
*/
static mut G_DMA_HEAP: [u8; BOARD_DMA_ALLOC_POOL_SIZE] = [0; BOARD_DMA_ALLOC_POOL_SIZE];

/****************************************************************************
* Public Functions
****************************************************************************/

/****************************************************************************
* Name: stm32_dma_alloc_init
*
* Description:
*   All boards may optionally provide this API to instantiate a pool of
*   memory for uses with FAST FS DMA operations.
*
****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn stm32_dma_alloc_init() -> cty::c_int {

    const GRANULE_SIZE: usize = 7; // 128B granule - must be > alignment (XXX bug?)
    const ALIGNMENT_SIZE: usize = 6; // 64B alignment 

    match ptr::NonNull::<cty::c_void>::new(gran_initialize(G_DMA_HEAP.as_mut_ptr(), size_of_val(&G_DMA_HEAP), GRANULE_SIZE, ALIGNMENT_SIZE)) {
        None => return -(ENOMEM as i32),
        Some(mut gran) => DMA_ALLOCATOR = gran.as_ptr()
    }
    OK
}

/* DMA-aware allocator stubs for the FAT filesystem. */
#[no_mangle]
pub unsafe extern "C" fn fat_dma_alloc(size: usize) -> *mut cty::c_void {
    gran_alloc(DMA_ALLOCATOR, size)
}

#[no_mangle]
pub unsafe extern "C" fn fat_dma_free(memory: *mut cty::c_void, size: usize) {
    gran_free(DMA_ALLOCATOR, memory, size);
}
