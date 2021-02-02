const wasm = require("./wasm/cryptli.js");
exports.cesar = wasm.cesar;
exports.binary_encrypt = wasm.binary_encrypt;
exports.binary_decrypt = wasm.binary_decrypt;
exports.numeral_encrypt = wasm.numeral_encrypt;
exports.numeral_decrypt = wasm.numeral_decrypt;
