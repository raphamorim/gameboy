#![cfg_attr(feature = "ffi", no_std)]

#[cfg(feature = "ffi")]
extern crate alloc;

#[cfg(feature = "ffi")]
use alloc::vec::Vec;

#[cfg(not(feature = "ffi"))]
use std::sync::Mutex;
#[cfg(not(feature = "ffi"))]
use std::sync::OnceLock;

#[cfg(all(feature = "ffi", not(test)))]
#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();



#[cfg(all(feature = "ffi", not(test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // In a no_std environment, we can't do much here
    // In a real implementation, you might write to a debug port or similar
    loop {}
}

// Initialize the allocator with some heap memory
#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn init_allocator(heap_start: *mut u8, heap_size: usize) {
    ALLOCATOR.lock().init(heap_start, heap_size);
}

// Required memory functions for no_std
#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // Copy backwards
        let mut i = n;
        while i > 0 {
            i -= 1;
            *dest.add(i) = *src.add(i);
        }
    } else {
        // Copy forwards
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
    }
    dest
}

#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}

#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[cfg(all(feature = "ffi", not(test)))]
#[no_mangle]
pub unsafe extern "C" fn bzero(s: *mut u8, n: usize) {
    memset(s, 0, n);
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod gameboy;
mod gpu;
mod input;
mod mbc;
mod mmu;
mod mode;
#[cfg(not(feature = "ffi"))]
mod screen;

// Prelude for no_std compatibility
#[cfg(feature = "ffi")]
#[allow(dead_code)]
mod prelude {
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
    pub use alloc::vec;
    pub use core::fmt;
}

pub use crate::input::KeypadKey;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    // screen::web::render().await
    // TODO: Process GL
}

#[cfg(not(feature = "ffi"))]
static GAMEBOY: OnceLock<Mutex<Option<crate::gameboy::Gameboy>>> = OnceLock::new();

#[cfg(feature = "ffi")]
static mut GAMEBOY: Option<crate::gameboy::Gameboy> = None;

#[cfg(target_vendor = "apple")]
unsafe impl Send for crate::gameboy::Gameboy {}
#[cfg(target_vendor = "apple")]
unsafe impl Sync for crate::gameboy::Gameboy {}

/// # Safety
///
/// This function is not safe due to from_raw_parts.
#[no_mangle]
pub unsafe extern "C" fn load(bytes: *const core::ffi::c_uchar, bytes_length: usize) {
    let bytes = core::slice::from_raw_parts(bytes, bytes_length);
    let bytes: Vec<u8> = Vec::from(bytes);
    
    #[cfg(not(feature = "ffi"))]
    {
        GAMEBOY
            .get_or_init(|| Some(crate::gameboy::Gameboy::new(bytes.to_vec(), None)).into());
    }
    
    #[cfg(feature = "ffi")]
    {
        GAMEBOY = Some(crate::gameboy::Gameboy::new(bytes.to_vec(), None));
    }
}

#[no_mangle]
pub extern "C" fn frame() {
    #[cfg(not(feature = "ffi"))]
    {
        if let Some(gb) = GAMEBOY.get() {
            if let Ok(mut locked_gb) = gb.lock() {
                if let Some(gameboy) = locked_gb.as_mut() {
                    gameboy.frame();
                };
            }
        }
    }
    
    #[cfg(feature = "ffi")]
    unsafe {
        if let Some(gameboy) = GAMEBOY.as_mut() {
            gameboy.frame();
        }
    }
}

#[no_mangle]
pub extern "C" fn keydown(key: KeypadKey) {
    #[cfg(not(feature = "ffi"))]
    {
        if let Some(gb) = GAMEBOY.get() {
            if let Ok(mut locked_gb) = gb.lock() {
                if let Some(gameboy) = locked_gb.as_mut() {
                    gameboy.keydown(key);
                }
            }
        }
    }
    
    #[cfg(feature = "ffi")]
    unsafe {
        if let Some(gameboy) = GAMEBOY.as_mut() {
            gameboy.keydown(key);
        }
    }
}

#[no_mangle]
pub extern "C" fn keyup(key: KeypadKey) {
    #[cfg(not(feature = "ffi"))]
    {
        if let Some(gb) = GAMEBOY.get() {
            if let Ok(mut locked_gb) = gb.lock() {
                if let Some(gameboy) = locked_gb.as_mut() {
                    gameboy.keyup(key);
                }
            }
        }
    }
    
    #[cfg(feature = "ffi")]
    unsafe {
        if let Some(gameboy) = GAMEBOY.as_mut() {
            gameboy.keyup(key);
        }
    }
}

#[repr(C)]
pub struct ImageBuffer {
    len: i32,
    data: *const u8,
}

#[no_mangle]
pub extern "C" fn image() -> ImageBuffer {
    #[cfg(not(feature = "ffi"))]
    {
        if let Some(gb) = GAMEBOY.get() {
            if let Ok(locked_gb) = gb.lock() {
                if let Some(gameboy) = locked_gb.as_ref() {
                    let image: &[u8] = gameboy.image();
                    let data = image.as_ptr();
                    let len = image.len() as i32;
                    // std::mem::forget(image);
                    // My guess image will be dropped but let's test

                    return ImageBuffer { len, data };
                }
            }
        }
    }
    
    #[cfg(feature = "ffi")]
    unsafe {
        if let Some(gameboy) = GAMEBOY.as_ref() {
            let image: &[u8] = gameboy.image();
            let data = image.as_ptr();
            let len = image.len() as i32;
            return ImageBuffer { len, data };
        }
    }

    ImageBuffer {
        len: 0,
        data: core::ptr::null(),
    }
}


