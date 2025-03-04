var wasm; // populated after instantiation
var canvas;
var gl;
var glObjects = [];

var env = {};
env.print = function (ptr) { console.log(get_string(ptr)); };
env.print_len = function (ptr, len) {
	console.log(new TextDecoder("utf8").decode(new Uint8Array(wasm.memory.buffer, ptr, len)));
};

function get_string(ptr) {
	var buf = new Uint8Array(wasm.memory.buffer, ptr);
	var len = buf.indexOf(0);
	var bytes = buf.slice(0, len);
	return new TextDecoder("utf8").decode(bytes);
}

function glObject(obj) {
	var index = glObjects.length;
	glObjects[index] = obj;
	return index;
}

function resizeCanvas() {
	var width = canvas.clientWidth;
	var height = canvas.clientHeight;
	if (canvas.width != width || canvas.height != height) {
		canvas.width = width;
		canvas.height = height;
		wasm.render();
	}
}

function glCompileShader(type, source_ptr) {
	var shader = gl.createShader(type);
	gl.shaderSource(shader, get_string(source_ptr));
	gl.compileShader(shader);
	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		const info = gl.getShaderInfoLog(shader);
		console.log(info);
	}
	return glObject(shader);
}

function glLinkShaderProgram(vert, frag) {
	program = gl.createProgram();
	gl.attachShader(program, glObjects[vert]);
	gl.attachShader(program, glObjects[frag]);
	gl.linkProgram(program);

	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		const info = gl.getProgramInfoLog(program);
		console.log(info);
	}
	return glObject(program);
}

function glGetAttribLocation(program, str_ptr) {
	var name = get_string(str_ptr);
	return gl.getAttribLocation(glObjects[program], name);
}

function glBufferData(target, size, data, usage) {
	var buffer = new Uint8Array(wasm.memory.buffer, data, size);
	gl.bufferData(target, buffer, usage);
}

function main() {
	canvas = document.getElementById("canvas");
	canvas.addEventListener("click", function (click) {
		wasm.mouse_click(
			(click.x * 2 - canvas.width) / canvas.height,
			-(click.y * 2 - canvas.height) / canvas.height,
		);
		wasm.render();
	});

	gl = canvas.getContext("webgl2");

	env.compile_shader = glCompileShader;
	env.link_shader_program = glLinkShaderProgram;
	env.use_program = (program) => gl.useProgram(glObjects[program]);
	env.get_attrib_location = glGetAttribLocation;

	env.canvas_width = () => gl.canvas.width;
	env.canvas_height = () => gl.canvas.height;

	env.viewport = (x, y, w, h) => gl.viewport(x, y, w, h);
	env.clear_color = (r, g, b, a) => gl.clearColor(r, g, b, a);
	env.clear = (mask) => gl.clear(mask);
	env.draw_arrays = (mode, ptr, count) => gl.drawArrays(mode, ptr, count);

	env.create_buffer = () => glObject(gl.createBuffer());
	env.bind_buffer = (target, buffer) => gl.bindBuffer(target, glObjects[buffer]);
	env.buffer_data = glBufferData;

	env.create_vertex_array = () => glObject(gl.createVertexArray());
	env.bind_vertex_array = (vao) => gl.bindVertexArray(glObjects[vao]);
	env.vertex_attrib_pointer = (index, size, type, normalized, stride, ptr) =>
		gl.vertexAttribPointer(index, size, type, normalized, stride, ptr);
	env.enable_vertex_attrib_array = (index) => gl.enableVertexAttribArray(index);

	env.get_uniform_location = (program, uniform) => 
		glObject(gl.getUniformLocation(glObjects[program], get_string(uniform)));
	env.uniform_2f = (loc, v0, v1) => gl.uniform2f(glObjects[loc], v0, v1);

	env.call_me = (f, arg) => wasm.__indirect_function_table.get(f)(arg);

	fetch("output.wasm").then(res => res.arrayBuffer()).then(function (bytes) {
		'use strict';
		var wasmBytes = new Uint8Array(bytes);
		WebAssembly.instantiate(wasmBytes, { env: env }).then(function (asm) {
			wasm = asm.instance.exports;
			wasm.init();
			resizeCanvas();
			wasm.render();
		});
	});
}

window.onload = main;
window.addEventListener('resize', resizeCanvas);
