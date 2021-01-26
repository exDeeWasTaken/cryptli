cargo build --release --target wasm32-unknown-unknown
<br>
wasm-bindgen --target deno --out-dir ./wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm
<br>
#Building
## Build raw wasm
### Run
`cargo build --release --target wasm32-unknown-unknown`<br>
## Create glue code for Deno
### Run
`wasm-bindgen --target deno --out-dir ./deno/wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm` <br>
### Change this line in `deno/wasm/cryptli.js`
const wasmFile = file.substring(0, file.lastIndexOf(Deno.build.os === 'windows' ? '\\' : '/') + 1) + 'wasm/cryptli_bg.wasm';