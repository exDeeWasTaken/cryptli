const {cesar, binary_encrypt, binary_decrypt, numeral_encrypt, numeral_decrypt} = require("../index.js");
const assert = require('assert');


describe("Cesar", function () {
    it("Encrypt", function () {
        assert.equal(cesar("aAzZ!.?`", 50), "yYxX!.?`");
    });
    it('Decrypt ', function () {
        assert.equal(cesar("yYxX!.?`", -50), "aAzZ!.?`");
    });
});

describe("Binary", function () {
    it("Encrypt", function () {
        assert.equal(binary_encrypt("I <3 cryptli!"), "1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001");
    });
    it('Decrypt ', function () {
        assert.equal(binary_decrypt("1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001"), "I <3 cryptli!");
    });
});

describe("Numeral", function () {
    it("Encrypt", function () {
        assert.equal(numeral_encrypt("I <3 cryptli!", 36), "21 w 1o 1f w 2r 36 3d 34 38 30 2x x");
    });
    it("Decrypt", function () {
        assert.equal(numeral_decrypt("21 w 1o 1f w 2r 36 3d 34 38 30 2x x", 36), "I <3 cryptli!");
    });
});