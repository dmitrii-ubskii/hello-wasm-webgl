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
