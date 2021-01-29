# Building
## Build raw wasm
### Run
$build_wasm = "cargo build --release --target wasm32-unknown-unknown";
Invoke-Expression $build_wasm;
## Create glue code for Deno
### Run
$build_deno_glue_code = "wasm-bindgen --target deno --out-dir ./deno/wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm";
Invoke-Expression $build_deno_glue_code;
### Change this line in `deno/wasm/cryptli.js`
################const wasmFile = file.substring(0, file.lastIndexOf(Deno.build.os === 'windows' ? '\\' : '/') + 1) + 'wasm/cryptli_bg.wasm';
## Create glue code for Node
### Run
$build_node_glue_code = "wasm-bindgen --target nodejs --out-dir ./node/wasm ./target/wasm32-unknown-unknown/release/cryptli.wasm";
Invoke-Expression $build_node_glue_code;

$path_to_deno_crytli_js = "./deno/wasm/cryptli.js";

$deno_crypli_js = Get-Content -Path $path_to_deno_crytli_js;

$edited = $deno_crypli_js.Replace("cryptli_bg.wasm", "wasm/cryptli_bg.wasm");

Set-Content -Path $path_to_deno_crytli_js -Value $edited;