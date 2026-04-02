#!/bin/bash
set -euo pipefail

# Source emsdk if not already in PATH
if ! command -v emcc &> /dev/null; then
    source "${EMSDK:-$HOME/emsdk}/emsdk_env.sh" 2>/dev/null
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TARGET_DIR="$ROOT_DIR/target/wasm32-unknown-emscripten/release"

echo "Building Rust staticlib..."
cargo build --target wasm32-unknown-emscripten --release -p scip-wasm

SCIP_LIB="$TARGET_DIR/build/scip-sys-*/out/lib/libscip.a"
SOPLEX_LIB="$TARGET_DIR/build/scip-sys-*/out/lib/libsoplex.a"
WASM_LIB="$TARGET_DIR/libscip_wasm.a"

echo "Linking with emcc..."
mkdir -p "$SCRIPT_DIR/dist"
emcc \
    $WASM_LIB \
    $SCIP_LIB \
    $SOPLEX_LIB \
    -fwasm-exceptions \
    -O3 \
    -o "$SCRIPT_DIR/dist/scip_wasm.js" \
    -sMODULARIZE=1 \
    -sEXPORT_NAME=SCIPWasm \
    -sALLOW_MEMORY_GROWTH=1 \
    -sEXPORTED_FUNCTIONS='["_scip_wasm_solve","_scip_wasm_get_obj_value","_scip_wasm_get_status","_scip_wasm_alloc","_scip_wasm_free","_malloc","_free"]' \
    -sEXPORTED_RUNTIME_METHODS='["ccall","cwrap","FS","UTF8ToString","stringToUTF8","lengthBytesUTF8"]' \
    -sFORCE_FILESYSTEM=1 \
    --no-entry

cp "$SCRIPT_DIR/dist/scip_wasm.js" "$SCRIPT_DIR/demo/"
cp "$SCRIPT_DIR/dist/scip_wasm.wasm" "$SCRIPT_DIR/demo/"

echo "Built: $SCRIPT_DIR/dist/scip_wasm.js"
echo "Built: $SCRIPT_DIR/dist/scip_wasm.wasm"
echo ""
echo "To serve the demo: python3 -m http.server 8080 -d $SCRIPT_DIR/demo"
