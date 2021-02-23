## Build raw wasm
cargo build --release --target wasm32-unknown-unknown;

## Create glue code for Deno
wasm-bindgen --target deno --out-dir ./deno/wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm


## Create glue code for Node
wasm-bindgen --target nodejs --out-dir ./node/wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm


## Change filepath in denos cryptli_js to match with the wasm-directory
$path_to_deno_crytli_js = "./deno/wasm/cryptli.js";
$deno_crypli_js = Get-Content -Path $path_to_deno_crytli_js;
$edited = $deno_crypli_js.Replace("'cryptli_bg.wasm'", "filePath");
$edited = $edited.Replace("const file = new URL(import.meta.url).pathname;", "const file = new URL(import.meta.url).pathname;
const filePath = Deno.build.os === 'windows' ? 'wasm/cryptli_bg.wasm' : 'cryptli_bg.wasm';")
Set-Content -Path $path_to_deno_crytli_js -Value $edited;