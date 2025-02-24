pub(super) mod gl {
	unsafe extern "C" {
		pub(crate) fn compile_shader(type_: u32, source: *const i8) -> usize;
		pub(crate) fn link_shader_program(vert: usize, frag: usize) -> usize;
		pub(crate) fn use_program(program: usize);
		pub(crate) fn get_attrib_location(
			shader_program: usize,
			ptr: *const u8,
			len: usize,
		) -> usize;
		pub(crate) fn canvas_width() -> u32;
		pub(crate) fn canvas_height() -> u32;
		pub(crate) fn viewport(x: u32, y: u32, width: u32, height: u32);
		pub(crate) fn clear_color(r: f32, g: f32, b: f32, a: f32);
		pub(crate) fn clear(mask: u32);
		pub(crate) fn draw_arrays(mode: u32, offset: u32, count: usize);

		pub(crate) fn create_buffer() -> usize;
		pub(crate) fn bind_buffer(target: u32, buffer: usize) -> usize;
		pub(crate) fn buffer_data(target: u32, size: usize, data: *const u8, usage: u32);

		pub(crate) fn create_vertex_array() -> usize;
		pub(crate) fn bind_vertex_array(vao: usize) -> usize;
		pub(crate) fn vertex_attrib_pointer(
			index: usize,
			size: usize,
			type_: u32,
			normalized: bool,
			stride: u32,
			pointer: usize,
		);
		pub(crate) fn enable_vertex_attrib_array(index: usize) -> usize;
	}

	pub(crate) const ARRAY_BUFFER: u32 = ::gl33::gl_enumerations::GL_ARRAY_BUFFER.0;
	pub(crate) const STATIC_DRAW: u32 = ::gl33::gl_enumerations::GL_STATIC_DRAW.0;

	pub(crate) const FLOAT: u32 = ::gl33::gl_enumerations::GL_FLOAT.0;

	pub(crate) const VERTEX_SHADER: u32 = ::gl33::gl_enumerations::GL_VERTEX_SHADER.0;
	pub(crate) const FRAGMENT_SHADER: u32 = ::gl33::gl_enumerations::GL_FRAGMENT_SHADER.0;

	pub(crate) const COLOR_BUFFER_BIT: u32 = ::gl33::gl_enumerations::GL_COLOR_BUFFER_BIT.0;
	pub(crate) const TRIANGLES: u32 = ::gl33::gl_enumerations::GL_TRIANGLES.0;
}

unsafe extern "C" {
	pub(super) fn print_len(ptr: *const u8, len: usize);
}
