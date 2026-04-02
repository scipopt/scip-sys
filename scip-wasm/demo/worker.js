importScripts('scip_wasm.js');

let Module = null;

SCIPWasm({
    print: function(text) {
        postMessage({type: 'log', line: text});
    },
    printErr: function(text) {
        postMessage({type: 'log', line: text});
    }
}).then(mod => {
    Module = mod;
    postMessage({type: 'ready'});
}).catch(err => {
    postMessage({type: 'error', message: 'Failed to load WASM: ' + err});
});

onmessage = async function(e) {
    if (e.data.type !== 'solve') return;

    let {buffer, filename} = e.data;
    let bytes = new Uint8Array(buffer);

    // Decompress .gz files in JS since SCIP is built without ZLIB
    if (filename.endsWith('.gz')) {
        const ds = new DecompressionStream('gzip');
        const writer = ds.writable.getWriter();
        writer.write(bytes);
        writer.close();
        bytes = new Uint8Array(await new Response(ds.readable).arrayBuffer());
        filename = filename.slice(0, -3); // strip .gz
    }

    Module.FS.writeFile(filename, bytes);

    const len = Module.lengthBytesUTF8(filename);
    const ptr = Module._scip_wasm_alloc(len + 1);
    Module.stringToUTF8(filename, ptr, len + 1);

    const retcode = Module._scip_wasm_solve(ptr, len);
    Module._scip_wasm_free(ptr, len + 1);

    const obj = Module._scip_wasm_get_obj_value();
    const status = Module._scip_wasm_get_status();
    postMessage({type: 'done', retcode, status, objective: obj});
};
