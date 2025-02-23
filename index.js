fetch("output.wasm").then(res => res.arrayBuffer()).then(function (wasmBytes) {
    'use strict';
    var wasm; // populated after instantiation

    function decode_string(offset, len) {
        var bytes = new Uint8Array(wasm.memory.buffer, offset, len);
        return new TextDecoder("utf8").decode(bytes);
    }

    WebAssembly.instantiate(
        new Uint8Array(wasmBytes), {
            env: {
                print: function (offset, len) { console.log(decode_string(offset, len)); },
                memory: new WebAssembly.Memory({ initial: 0 }),
            }
        }
    ).then(function (asm) {
        wasm = asm.instance.exports;
        wasm.run();
    });
});

var canvas;
var gl;
var vao;

function render() {
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
};

function resizeCanvas() {
    var width = canvas.clientWidth;
    var height = canvas.clientHeight;
    if (canvas.width != width ||
        canvas.height != height) {
        canvas.width = width;
        canvas.height = height;
        render();
    }
}

window.onload = function () {
    canvas = document.getElementById("canvas");

    gl = canvas.getContext("webgl2");

    var vert = gl.createShader(gl.VERTEX_SHADER);
    gl.shaderSource(
        vert,
        `#version 300 es
        in vec4 position;
        void main() {
            gl_Position = position;
        }`
    );
    gl.compileShader(vert);

    var frag = gl.createShader(gl.FRAGMENT_SHADER);
    gl.shaderSource(
        frag,
        `#version 300 es
        precision highp float;
        out vec4 outColor;
        void main() {
            outColor = vec4(1, 1, 1, 1);
        }`
    );
    gl.compileShader(frag);

    shader = gl.createProgram();
    gl.attachShader(shader, vert);
    gl.attachShader(shader, frag);
    gl.linkProgram(shader);
    gl.useProgram(shader);

    let positions = new Float32Array(9);
    positions[0] = -0.7;
    positions[1] = -0.6;
    positions[2] =  0.0;
    positions[3] =  0.7;
    positions[4] = -0.6;
    positions[5] =  0.0;
    positions[6] =  0.0;
    positions[7] =  0.6;
    positions[8] =  0.0;

    let pos = gl.getAttribLocation(shader, "position");
    let buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

    vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    gl.vertexAttribPointer(pos, 3, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(pos);

    resizeCanvas();
    render();
}

window.addEventListener('resize', resizeCanvas);
