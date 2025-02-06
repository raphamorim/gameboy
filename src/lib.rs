use std::sync::Mutex;
use std::sync::OnceLock;

#[cfg(all(target_arch = "wasm32", feature = "ffi"))]
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

pub use crate::input::KeypadKey;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    // screen::web::render().await
    // TODO: Process GL
}

static GAMEBOY: OnceLock<Mutex<Option<crate::gameboy::Gameboy>>> = OnceLock::new();

#[cfg(target_vendor = "apple")]
unsafe impl Send for crate::gameboy::Gameboy {}
#[cfg(target_vendor = "apple")]
unsafe impl Sync for crate::gameboy::Gameboy {}

/// # Safety
///
/// This function is not safe due to from_raw_parts.
#[no_mangle]
pub unsafe extern "C" fn load(bytes: *const std::ffi::c_uchar, bytes_length: usize) {
    let bytes = std::slice::from_raw_parts(bytes, bytes_length);
    let bytes: Vec<u8> = Vec::from(bytes);
    GAMEBOY
        .get_or_init(|| Some(crate::gameboy::Gameboy::new(bytes.to_vec(), None)).into());
}

#[no_mangle]
pub extern "C" fn frame() {
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            locked_gb.as_mut().unwrap().frame();
        }
    }
}

#[no_mangle]
pub extern "C" fn keydown(key: KeypadKey) {
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            locked_gb.as_mut().unwrap().keydown(key);
        }
    }
}

#[no_mangle]
pub extern "C" fn keyup(key: KeypadKey) {
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            locked_gb.as_mut().unwrap().keyup(key);
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
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            let image: &[u8] = locked_gb.as_mut().unwrap().image();
            let data = image.as_ptr();
            let len = image.len() as i32;
            // std::mem::forget(image);
            // My guess image will be dropped but let's test

            return ImageBuffer { len, data };
        }
    }

    ImageBuffer {
        len: 0,
        data: std::ptr::null_mut(),
    }
}
