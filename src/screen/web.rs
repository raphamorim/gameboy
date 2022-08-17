extern crate console_error_panic_hook;

use crate::gameboy::Gameboy;
use crate::input::Button;

use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{CanvasRenderingContext2d, ImageData};

use std::panic;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn add_event_listener(listener: &str, f: &Closure<dyn FnMut(web_sys::KeyboardEvent)>) {
    window()
        .add_event_listener_with_callback(listener, f.as_ref().unchecked_ref())
        .expect("should register `add_event_listener` OK");
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// TODO: Move to WebGL tex2d
#[wasm_bindgen]
pub async fn render(rom: Vec<u8>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let game = document.get_element_by_id("game");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    game.unwrap().append_child(&canvas)?;
    canvas.set_width(160);
    canvas.set_height(144);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut gb = Gameboy::new();

    gb.load_rom_with_u8_vec(rom);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    // let mut i = 0;
    let current_key_code: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    gb.frame();

    {
        let current_key_code = current_key_code.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // if i >= 10 {
            //     let _ = f.borrow_mut().take();
            //     return;
            // }

            // i += 1;

            let key: RefMut<_> = current_key_code.borrow_mut();

            // log_u32(*key);
            match *key {
                // A
                65 => gb.keydown(Button::A),
                -65 => gb.keyup(Button::A),
                // S
                83 => gb.keydown(Button::B),
                -83 => gb.keyup(Button::B),
                // Z
                90 => gb.keydown(Button::Select),
                -90 => gb.keyup(Button::Select),
                // X
                88 => gb.keydown(Button::Start),
                -88 => gb.keyup(Button::Start),

                // Left
                37 => gb.keydown(Button::Left),
                -37 => gb.keyup(Button::Left),
                // Right
                39 => gb.keydown(Button::Right),
                -39 => gb.keyup(Button::Right),
                // Up
                38 => gb.keydown(Button::Up),
                -38 => gb.keyup(Button::Up),
                // Down
                40 => gb.keydown(Button::Down),
                -40 => gb.keyup(Button::Down),
                _ => (),
            }

            gb.frame();
            let data: &mut [u8] = gb.image_mut();
            let _image_data = match ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(data),
                160,
                144,
            ) {
                Ok(d) => {
                    context.put_image_data(&d, 0.0, 0.0).ok();
                }
                Err(err) => {
                    log(format!("{:?}", err));
                }
            };

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    }
    {
        let current_key_code = current_key_code.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key_code();
            *current_key_code.borrow_mut() = key as i32;
        });
        window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let current_key_code = current_key_code.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key_code();
            *current_key_code.borrow_mut() = (key as i32) * -1;
        });
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

// pub fn compile_shader(
//     context: &WebGlRenderingContext,
//     shader_type: u32,
//     source: &str,
// ) -> Result<WebGlShader, String> {
//     let shader = context
//         .create_shader(shader_type)
//         .ok_or_else(|| String::from("Unable to create shader object"))?;
//     context.shader_source(&shader, source);
//     context.compile_shader(&shader);

//     if context
//         .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(shader)
//     } else {
//         Err(context
//             .get_shader_info_log(&shader)
//             .unwrap_or_else(|| String::from("Unknown error creating shader")))
//     }
// }

// pub fn link_program(
//     context: &WebGlRenderingContext,
//     vert_shader: &WebGlShader,
//     frag_shader: &WebGlShader,
// ) -> Result<WebGlProgram, String> {
//     let program = context
//         .create_program()
//         .ok_or_else(|| String::from("Unable to create shader object"))?;

//     context.attach_shader(&program, vert_shader);
//     context.attach_shader(&program, frag_shader);
//     context.link_program(&program);

//     if context
//         .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(program)
//     } else {
//         Err(context
//             .get_program_info_log(&program)
//             .unwrap_or_else(|| String::from("Unknown error creating program object")))
//     }
// }
