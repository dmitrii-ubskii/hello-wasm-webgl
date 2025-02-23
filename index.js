var wasm; // populated after instantiation
var canvas;
var gl;
var glObjects = [];

var env = {};
env.print = function (offset, len) { console.log(decode_string(offset, len)); };

function decode_string(offset, len) {
    var bytes = new Uint8Array(wasm.memory.buffer, offset, len);
    return new TextDecoder("utf8").decode(bytes);
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

function glCompileShader(type, source_offset, source_len) {
    var shader = gl.createShader(type);
    gl.shaderSource(shader, decode_string(source_offset, source_len));
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        const info = gl.getShaderInfoLog(shader);
        console.log(info);
    }

    var index = glObjects.length;
    glObjects[index] = shader;
    return index;
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

    var index = glObjects.length;
    glObjects[index] = program;
    return index;
}

function glCreateBuffer() {
    var buffer = gl.createBuffer();
    var index = glObjects.length;
    glObjects[index] = buffer;
    return index;
}

function glCreateVertexArray() {
    var vao = gl.createVertexArray();
    var index = glObjects.length;
    glObjects[index] = vao;
    return index;
}

function glGetAttribLocation(program, str_offset, str_len) {
    var name = decode_string(str_offset, str_len);
    return gl.getAttribLocation(glObjects[program], name);
}

function glVertexAttribPointer(index, size, type, normalized, stride, ptr) {
    gl.vertexAttribPointer(index, size, type, normalized, stride, ptr);
}

function glBufferData(target, size, data, usage) {
    var buffer = new Float32Array(wasm.memory.buffer, data, size / 4);
    gl.bufferData(target, buffer, usage);
}

function main() {
    canvas = document.getElementById("canvas");
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
    env.draw_arrays = (mode, offset, count) => gl.drawArrays(mode, offset, count);

	env.create_buffer = glCreateBuffer;
	env.bind_buffer = (target, buffer) => gl.bindBuffer(target, glObjects[buffer]);
	env.buffer_data = glBufferData;

	env.create_vertex_array = glCreateVertexArray;
	env.bind_vertex_array = (vao) => gl.bindVertexArray(glObjects[vao]);
	env.vertex_attrib_pointer = glVertexAttribPointer;
	env.enable_vertex_attrib_array = (index) => gl.enableVertexAttribArray(index);

    fetch("output.wasm").then(res => res.arrayBuffer()).then(function (bytes) {
        'use strict';
        var wasmBytes = new Uint8Array(bytes);
        env.memory = new WebAssembly.Memory({ initial: 0 });
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
