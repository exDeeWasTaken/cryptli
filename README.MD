cargo build --release --target wasm32-unknown-unknown
<br>
wasm-bindgen --target deno --out-dir ./wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm
<br>
const realFile = `${Deno.cwd()}/wasm/${wasmFile}`;
<br>
const wasmModule = new WebAssembly.Module(Deno.readFileSync(realFile));