#![no_std]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
	if let Some(message) = info.message().as_str() {
		print(message);
	}
	loop {}
}

mod js;

fn print(str: &str) {
	let (ptr, len) = str_into_parts(str);
	unsafe { js::print(ptr, len) };
}

fn str_into_parts(str: &str) -> (*const u8, usize) {
	(str.as_ptr(), str.len())
}

#[rustfmt::skip]
const TRIANGLE: [f32; 9] = [
	-0.7, -0.6, 0.0,
	 0.7, -0.6, 0.0,
	 0.0,  0.6, 0.0,
];

#[unsafe(no_mangle)]
pub extern "C" fn init() {
	let vert = compile_shader(
		js::gl::VERTEX_SHADER,
		"#version 300 es
		in vec4 position;
		void main() {
			gl_Position = position;
		}",
	);

	let frag = compile_shader(
		js::gl::FRAGMENT_SHADER,
		"#version 300 es
        precision highp float;
        out vec4 outColor;
        void main() {
            outColor = vec4(gl_FragCoord.xyx * 0.001, 1);
        }",
	);

	let shader_program = link_shader_program(vert, frag);
	unsafe { js::gl::use_program(shader_program) };

	let position_attr_location = get_attrib_location(shader_program, "position");

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
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn render() {
	unsafe {
		js::gl::viewport(0, 0, js::gl::canvas_width(), js::gl::canvas_height());
		js::gl::clear_color(0.0, 0.0, 0.0, 1.0);
		js::gl::clear(js::gl::COLOR_BUFFER_BIT);
		js::gl::draw_arrays(js::gl::TRIANGLES, 0, 3);
	}
}

fn link_shader_program(vert: usize, frag: usize) -> usize {
	unsafe { js::gl::link_shader_program(vert, frag) }
}

fn compile_shader(shader_type: u32, source: &str) -> usize {
	let (ptr, len) = str_into_parts(source);
	unsafe { js::gl::compile_shader(shader_type, ptr, len) }
}

fn get_attrib_location(shader_program: usize, attribute_name: &str) -> usize {
	let (ptr, len) = str_into_parts(attribute_name);
	unsafe { js::gl::get_attrib_location(shader_program, ptr, len) }
}
