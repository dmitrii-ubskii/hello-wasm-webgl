#![no_std]
extern crate alloc;

mod allocator;
mod atomic;
mod js;

use alloc::string::String;
use core::{
	arch::wasm32::unreachable,
	ffi::CStr,
	ptr::null,
	sync::atomic::{AtomicUsize, Ordering},
};

use atomic::AtomicF32Array;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
	print_str("Panic!");
	if let Some(message) = info.message().as_str() {
		print_str(message);
	}
	unreachable()
}

fn print_str(str: &str) {
	unsafe { js::print_len(str.as_ptr(), str.len()) };
}

#[rustfmt::skip]
static TRIANGLE: AtomicF32Array<9> = AtomicF32Array::new([
	-0.7, -0.6, 0.0,
	 0.7, -0.6, 0.0,
	 0.0,  0.6, 0.0,
]);
static VERTEX: AtomicUsize = AtomicUsize::new(0);

static SHADER: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn f(_: *const u8) {
	print_str("f");
}

#[unsafe(no_mangle)]
pub extern "C" fn g(_: *const u8) {
	let s = String::from("hello");
	print_str(&s);
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {
	unsafe { js::call_me(f, null()) };
	unsafe { js::call_me(g, null()) };

	let vert = compile_shader(
		js::gl::VERTEX_SHADER,
		c"#version 300 es
		in vec4 position;
		uniform vec2 screen_size;
		void main() {
			float aspect_ratio = screen_size.y / screen_size.x;
			gl_Position = vec4(position.x * aspect_ratio, position.y, position.zw);
		}",
	);

	let frag = compile_shader(
		js::gl::FRAGMENT_SHADER,
		c"#version 300 es
		precision highp float;
		uniform vec2 screen_size;
		out vec4 outColor;
		void main() {
			vec2 middle = screen_size * 0.5;
			float r = min(middle.x, middle.y);
			outColor = vec4((gl_FragCoord.xy - middle + vec2(r, r)) / r, 0, 1);
		}",
	);

	let shader_program = link_shader_program(vert, frag);
	SHADER.store(shader_program, Ordering::Relaxed);
	unsafe { js::gl::use_program(shader_program) };
}

#[unsafe(no_mangle)]
pub extern "C" fn mouse_click(x: f32, y: f32) {
	let i = VERTEX.fetch_add(1, Ordering::Relaxed) % 3;

	TRIANGLE.store(3 * i, x);
	TRIANGLE.store(3 * i + 1, y);
}

#[unsafe(no_mangle)]
pub extern "C" fn render() {
	let shader_program = SHADER.load(Ordering::Relaxed);
	let position_attr_location = get_attrib_location(shader_program, c"position");

	unsafe {
		let buffer = js::gl::create_buffer();
		js::gl::bind_buffer(js::gl::ARRAY_BUFFER, buffer);

		let positions_ptr = TRIANGLE.as_ptr().cast();
		let positions_size = core::mem::size_of_val(&TRIANGLE);
		js::gl::buffer_data(
			js::gl::ARRAY_BUFFER,
			positions_size,
			positions_ptr,
			js::gl::STATIC_DRAW,
		);

		let vao = js::gl::create_vertex_array();
		js::gl::bind_vertex_array(vao);

		js::gl::vertex_attrib_pointer(position_attr_location, 3, js::gl::FLOAT, false, 0, 0);
		js::gl::enable_vertex_attrib_array(position_attr_location);

		let location = js::gl::get_uniform_location(shader_program, c"screen_size".as_ptr());
		js::gl::uniform_2f(location, js::gl::canvas_width() as f32, js::gl::canvas_height() as f32);
		js::gl::viewport(0, 0, js::gl::canvas_width(), js::gl::canvas_height());
		js::gl::clear_color(0.0, 0.0, 0.0, 1.0);
		js::gl::clear(js::gl::COLOR_BUFFER_BIT);
		js::gl::draw_arrays(js::gl::TRIANGLES, 0, 3);
	}
}

fn link_shader_program(vert: usize, frag: usize) -> usize {
	unsafe { js::gl::link_shader_program(vert, frag) }
}

fn compile_shader(shader_type: u32, source: &CStr) -> usize {
	unsafe { js::gl::compile_shader(shader_type, source.as_ptr()) }
}

fn get_attrib_location(shader_program: usize, attribute_name: &CStr) -> usize {
	unsafe { js::gl::get_attrib_location(shader_program, attribute_name.as_ptr()) }
}
