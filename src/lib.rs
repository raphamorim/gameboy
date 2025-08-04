use std::sync::Mutex;
use std::sync::OnceLock;

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
#[cfg(not(target_arch = "wasm32"))]
pub mod sound;

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
            if let Some(gameboy) = locked_gb.as_mut() {
                gameboy.frame();
            };
        }
    }
}

#[no_mangle]
pub extern "C" fn keydown(key: KeypadKey) {
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            if let Some(gameboy) = locked_gb.as_mut() {
                gameboy.keydown(key);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn keyup(key: KeypadKey) {
    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            if let Some(gameboy) = locked_gb.as_mut() {
                gameboy.keyup(key);
            }
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

    ImageBuffer {
        len: 0,
        data: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[cfg(feature = "ffi-base64")]
pub extern "C" fn image_base64() -> *const std::os::raw::c_char {
    use base64::{engine::general_purpose, Engine};
    use image::DynamicImage;
    use std::ffi::CString;
    use std::io::Cursor;

    if let Some(gb) = GAMEBOY.get() {
        if let Ok(mut locked_gb) = gb.lock() {
            let width = 160;
            let height = 144;

            let image: &[u8] = locked_gb.as_mut().unwrap().image();
            // std::mem::forget(image);

            // Allocate a new buffer for the RGB image, 3 bytes per pixel
            let mut output_data = vec![0u8; width as usize * height as usize * 3];

            let mut i = 0;
            // Iterate through 4-byte chunks of the image data (RGBA bytes)
            for chunk in image.chunks(4) {
                // ... and copy each of them to output, leaving out the A byte
                output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
                i += 3;
            }

            let buffer =
                image::ImageBuffer::from_raw(width, height, output_data).unwrap();
            let img: DynamicImage = image::DynamicImage::ImageRgb8(buffer);
            // if scale > 1 {
            //     buffer = image::imageops::resize(
            //         &buffer,
            //         width * (scale as u32),
            //         height * (scale as u32),
            //         image::imageops::FilterType::Nearest,
            //     );
            // }

            let mut png: Vec<u8> = vec![];
            img.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
                .expect("don't fail img write_to");
            let data = general_purpose::STANDARD.encode(&png);

            // let cstring_data = CString::new(img.as_bytes()).expect("don't fail");
            // return cstring_data.into_raw();

            // let mut png: Vec<u8> = vec![];
            // img.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png).expect("don't fail img write_to");
            // let data = general_purpose::STANDARD.encode(&png);

            let cstring_data = CString::new(data).expect("don't fail");
            return cstring_data.into_raw();
        }
    }

    let data = CString::new("").expect("don't fail");
    data.into_raw()
}
