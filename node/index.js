const wasm = require("./wasm/cryptli.js");
exports.cesar = wasm.cesar;
exports.binary_encrypt = wasm.binary_encrypt;
exports.binary_decrypt = wasm.binary_decrypt;
exports.numeral_encrypt = wasm.numeral_encrypt;
exports.numeral_decrypt = wasm.numeral_decrypt;
exports.ascii_encrypt = wasm.ascii_encrypt;
exports.ascii_decrypt = wasm.ascii_decrypt;
exports.morse_encrypt = wasm.morse_encrypt;
exports.morse_decrypt = wasm.morse_decrypt;
exports.vigenere_encrypt = wasm.vigenere_encrypt;
exports.vigenere_decrypt = wasm.vigenere_decrypt;
exports.atbash_encrypt = wasm.atbash_encrypt;
exports.atbash_decrypt = wasm.atbash_encrypt;
exports.latin_encrypt = wasm.latin_encrypt;
exports.scytale_encrypt = wasm.scytale_encrypt;
exports.scytale_decrypt = wasm.scytale_decrypt;

