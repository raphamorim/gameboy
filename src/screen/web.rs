extern crate console_error_panic_hook;

use crate::gameboy::Gameboy;
use crate::screen::debug_rom;
// use core::cell::RefCell;

// use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, HtmlInputElement, ImageBitmap, MouseEvent, WebGlProgram, WebGlRenderingContext,
    WebGlShader,
};

use std::panic;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("failed to create image bitmap: {0:?}")]
    CreateImageBitmap(JsValue),

    #[error("failed to await image bitmap: {0:?}")]
    AwaitCreateImageBitmap(JsValue),

    #[error("failed to retrieve image bitmap length: {0:?}")]
    MappedDataLength(JsValue),

    #[error("failed to map image bitmap data: {0:?}")]
    MapData(JsValue),

    #[error("failed to await image bitmap data: {0:?}")]
    AwaitMapData(JsValue),
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

async fn from_data(data: &mut [u8]) -> Result<ImageBitmap, LoadError> {
    let blob = Blob::new().unwrap();
    let image_bitmap: ImageBitmap = {
        let promise = web_sys::window()
            .unwrap()
            .create_image_bitmap_with_blob(&blob)
            .map_err(LoadError::CreateImageBitmap)?;

        let value = JsFuture::from(promise)
            .await
            .map_err(LoadError::AwaitCreateImageBitmap)?;

        assert!(value.is_instance_of::<ImageBitmap>());
        value.dyn_into().unwrap()
    };

    Ok(image_bitmap)

    // Self::from_image_bitmap(gl, image_bitmap, params).await
}

pub async fn render() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(160);
    canvas.set_height(144);
    // canvas.style().set_property("border", "solid")?;

    // let context = canvas
    //     .get_context("webgl")?
    //     .unwrap()
    //     .dyn_into::<WebGlRenderingContext>()?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // let vert_shader = compile_shader(
    //     &context,
    //     WebGlRenderingContext::VERTEX_SHADER,
    //     r#"
    //     attribute vec4 position;
    //     void main() {
    //         gl_Position = position;
    //     }
    // "#,
    // )?;
    // let frag_shader = compile_shader(
    //     &context,
    //     WebGlRenderingContext::FRAGMENT_SHADER,
    //     r#"
    //     void main() {
    //         gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    //     }
    // "#,
    // )?;
    // let program = link_program(&context, &vert_shader, &frag_shader)?;
    // context.use_program(Some(&program));

    // let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    // let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    // context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // // Note that `Float32Array::view` is somewhat dangerous (hence the
    // // `unsafe`!). This is creating a raw view into our module's
    // // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // // causing the `Float32Array` to be invalid.
    // //
    // // As a result, after `Float32Array::view` we have to be very careful not to
    // // do any memory allocations before it's dropped.
    // unsafe {
    //     let vert_array = js_sys::Float32Array::view(&vertices);

    //     context.buffer_data_with_array_buffer_view(
    //         WebGlRenderingContext::ARRAY_BUFFER,
    //         &vert_array,
    //         WebGlRenderingContext::STATIC_DRAW,
    //     );
    // }

    // context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    // context.enable_vertex_attrib_array(0);

    let mut gb = Gameboy::new();
    let rom: Vec<u8> = debug_rom::get_rom();

    gb.load_rom_with_u8_vec(rom);

    // one CPU loop

    // let f = Rc::new(RefCell::new(None));
    //    let g = f.clone();

    //    let mut i = 0;
    //    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //        if i > 10 {
    //            // body().set_text_content(Some("All done!"));

    //            // Drop our handle to this closure so that it will get cleaned
    //            // up once we return.
    //            let _ = f.borrow_mut().take();
    //            // return;
    //        }

    // i += 1;

    gb.frame();
    let image_data: &mut [u8] = gb.image_mut();

    log(format!("{:?}", image_data));
    // log_u32(i);

    // let image = w.create_image_bitmap_with_u8_array(&mut data).unwrap();
    // let image_bitmap = from_data(image_data);
    // context.draw_image_with_image_bitmap(&image_bitmap.await.unwrap(), 160.0, 144.0)?;

    // context.clear_color(0.0, 0.0, 0.0, 1.0);
    // context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // context.compressed_tex_image_2d_with_u8_array(
    // 	WebGlRenderingContext::TEXTURE_2D,
    // 	WebGlRenderingContext::RGB as i32,
    // 	160,
    // 	144,
    // 	0,
    // 	0,
    // 	data,
    // );

    // context.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, 6, WebGlRenderingContext::UNSIGNED_INT, 1);

    //     request_animation_frame(f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));

    // request_animation_frame(g.borrow().as_ref().unwrap());

    // click não ta rolando

    //   {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let input = document.get_element_by_id("run").unwrap();

    // let context = context.clone();
    // let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
    // 	let mut gb = gameboy::Gameboy::new();
    //     // let rom: Vec<u8> = gb.read_rom_by_filepath("./sample-rom.gb");

    // 	let rom: Vec<u8> = debug_rom::get_rom();

    // 	gb.load(rom);

    // 	// one CPU loop
    // 	gb.frame();

    // 	let data = gb.image();

    // 	println!("{:?}", data);

    // 	context.clear_color(0.0, 0.0, 0.0, 1.0);
    // 	context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // 	context.compressed_tex_image_2d_with_u8_array(
    // 		WebGlRenderingContext::TEXTURE_2D,
    // 		WebGlRenderingContext::RGB as i32,
    // 		160,
    // 		144,
    // 		0,
    // 		0,
    // 		data,
    // 	);

    // 	context.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, 6, WebGlRenderingContext::UNSIGNED_INT, 1);
    //       }) as Box<dyn FnMut(_)>);
    //       input.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    //       closure.forget();
    //   }

    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}
