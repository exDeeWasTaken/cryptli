const {cesar, binary_encrypt, binary_decrypt} = require("../index.js");
const assert = require('assert');



describe("Cesar", function() {
    it("Encrypt", function() {
        assert.equal(cesar("aAzZ!.?`", 50), "yYxX!.?`");
    });
    it('Decrypt ', function () {
        assert.equal(cesar("yYxX!.?`", -50), "aAzZ!.?`");
    });
});

describe("Binary", function() {
    it("Encrypt", function() {
        assert.equal(binary_encrypt("I <3 cryptli!"), "1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001");
    });
    it('Decrypt ', function () {
        assert.equal(binary_decrypt("1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001"), "I <3 cryptli!");
    });
});