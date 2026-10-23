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

static LAST_ERROR: Mutex<Option<std::ffi::CString>> = Mutex::new(None);

fn set_last_error(message: &str) {
    if let Ok(mut last) = LAST_ERROR.lock() {
        *last = std::ffi::CString::new(message).ok();
    }
}

/// The message for the most recent failed call, or null. Valid until the
/// next failure.
#[no_mangle]
pub extern "C" fn last_error() -> *const std::ffi::c_char {
    match LAST_ERROR.lock() {
        Ok(last) => last.as_ref().map_or(std::ptr::null(), |m| m.as_ptr()),
        Err(_) => std::ptr::null(),
    }
}

/// Like `load`, but battery-backed cartridges keep their RAM in a `.gbsave`
/// file next to `rom_path` (the path only names the save; the ROM is not
/// read from it). Pass null for no saves.
///
/// Returns 0 on success, 1 if a game is already loaded, -1 for invalid
/// arguments and -2 if the core rejected the cartridge (see `last_error`).
///
/// # Safety
///
/// `bytes` must point to `bytes_length` readable bytes, and `rom_path` must be
/// null or a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn load_with_save(
    bytes: *const std::ffi::c_uchar,
    bytes_length: usize,
    rom_path: *const std::ffi::c_char,
) -> i32 {
    if bytes.is_null() {
        return -1;
    }
    if GAMEBOY.get().is_some() {
        return 1;
    }
    let data = std::slice::from_raw_parts(bytes, bytes_length).to_vec();
    let path = if rom_path.is_null() {
        None
    } else {
        match std::ffi::CStr::from_ptr(rom_path).to_str() {
            Ok(p) => Some(std::path::PathBuf::from(p)),
            Err(_) => return -1,
        }
    };
    match crate::gameboy::Gameboy::try_new(data, path) {
        Ok(gameboy) => match GAMEBOY.set(Some(gameboy).into()) {
            Ok(()) => 0,
            Err(_) => 1,
        },
        Err(e) => {
            set_last_error(e);
            -2
        }
    }
}

/// Writes battery-backed cartridge RAM to its save file now. Safe to call
/// often; cartridges without a battery do nothing.
///
/// Returns 0 on success, -1 if no game is loaded and -2 on a write error
/// (see `last_error`).
#[no_mangle]
pub extern "C" fn save() -> i32 {
    let Some(gb) = GAMEBOY.get() else { return -1 };
    let Ok(locked_gb) = gb.lock() else { return -1 };
    let Some(gameboy) = locked_gb.as_ref() else {
        return -1;
    };
    match gameboy.save() {
        Ok(()) => 0,
        Err(e) => {
            set_last_error(e);
            -2
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod ffi_audio {
    use crate::sound::cpal_audio::FrameQueue;
    use std::cell::RefCell;
    use std::sync::atomic::AtomicU32;
    use std::sync::OnceLock;

    pub static QUEUE: OnceLock<FrameQueue> = OnceLock::new();
    pub static SAMPLE_RATE: AtomicU32 = AtomicU32::new(0);

    thread_local! {
        // The output stream stops when dropped, so it lives as long as the
        // thread that started it (the host's main thread).
        pub static STREAM: RefCell<Option<cpal::Stream>> = const { RefCell::new(None) };
    }
}

/// Starts sound on the default output device. Call after loading a game, from
/// the thread that will keep running (usually the main thread).
///
/// Returns the output sample rate in Hz, or 0 if audio could not start.
#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn enable_audio() -> u32 {
    use crate::sound::AudioPlayer;
    use std::sync::atomic::Ordering;

    if ffi_audio::QUEUE.get().is_some() {
        return ffi_audio::SAMPLE_RATE.load(Ordering::Relaxed);
    }
    let Some(gb) = GAMEBOY.get() else { return 0 };
    let Ok(mut locked_gb) = gb.lock() else {
        return 0;
    };
    let Some(gameboy) = locked_gb.as_mut() else {
        return 0;
    };
    let Some((player, stream)) = crate::sound::cpal_audio::CpalPlayer::new() else {
        set_last_error("No usable audio output device");
        return 0;
    };

    let rate = player.samples_rate();
    let queue = player.queue();
    gameboy.enable_audio(Box::new(player));
    ffi_audio::STREAM.with(|s| *s.borrow_mut() = Some(stream));
    ffi_audio::SAMPLE_RATE.store(rate, Ordering::Relaxed);
    let _ = ffi_audio::QUEUE.set(queue);
    rate
}

/// Stereo frames produced but not yet played. Divide by the sample rate for
/// latency; hosts use it to pace `frame()` to the audio clock.
#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn audio_queued() -> u32 {
    ffi_audio::QUEUE
        .get()
        .and_then(|q| q.lock().ok().map(|q| q.len() as u32))
        .unwrap_or(0)
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
