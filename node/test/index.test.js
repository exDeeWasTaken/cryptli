const {
    cesar,
    binary_encrypt,
    binary_decrypt,
    numeral_encrypt,
    numeral_decrypt,
    ascii_encrypt,
    ascii_decrypt,
    morse_encrypt,
    morse_decrypt,
    vigenere_encrypt,
    vigenere_decrypt,
    atbash_encrypt,
    atbash_decrypt,
    latin_encrypt,
    scytale_encrypt,
    scytale_decrypt,
    rotation_encrypt,
    rotation_decrypt
} = require("../index.js");
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

describe("Ascii", function () {
    it("Encrypt", function () {
        assert.equal(ascii_encrypt("I <3 cryptli!"), "73 32 60 51 32 99 114 121 112 116 108 105 33");
    });
    it("Decrypt", function () {
        assert.equal(ascii_decrypt("73 32 60 51 32 99 114 121 112 116 108 105 33"), "I <3 cryptli!");
    });
});

describe("Morse", function () {
    it("Encrypt", function () {
        assert.equal(morse_encrypt("abcD"), ".- -... -.-. -..");
    });
    it("Decrypt", function () {
        assert.equal(morse_decrypt(".- -... -.-. -.."), "abcd");
    });
});

describe("Vigenere", function () {
    it("Encrypt", function () {
        assert.equal(vigenere_encrypt("ILOVECRIPTLY", "KEY"), "SPMFIABMNDPW");
    });
    it("Decrypt", function () {
        assert.equal(vigenere_decrypt("SPMFIABMNDPW", "KEY"), "ILOVECRIPTLY")
    });
});

describe("Atbash", function () {
    it("Encrypt", function () {
        assert.equal(atbash_encrypt("ILOVECRIPTLY"), "ROLEVXIRKGOB");
    });
    it("Decrypt", function () {
        assert.equal(atbash_decrypt("ROLEVXIRKGOB"), "ILOVECRIPTLY")
    });
});

describe("Latin", function () {
    it("Encrypt", function () {
        assert.equal(latin_encrypt("I love cryptli"), "9   12 15 22 5   3 18 25 16 20 12 9");
    });
    // it("Decrypt",function(){
    //     assert.equal(atbash_decrypt("ROLEVXIRKGOB"), "ILOVECRIPTLY")
    // });
});

describe("Scytale", function () {
    it("Encrypt", function () {
        assert.equal(scytale_encrypt("abcdefghijklmnopqrstuvwxyzBumBumBum", 3), "adgjmpsvyuuubehknqtwzmmmcfiloruxBBB");
    });
    it("Decrypt", function () {
        assert.equal(scytale_decrypt("adgjmpsvyuuubehknqtwzmmmcfiloruxBBB", 3), "abcdefghijklmnopqrstuvwxyzBumBumBum")
    });
});

describe("Rotation", function () {
    it("Encrypt", function () {
        assert.equal(rotation_encrypt("ILOVECRIPTLY", 13), "VYBIRPEVCGYL");
    });
    it("Decrypt", function () {
        assert.equal(rotation_decrypt("VYBIRPEVCGYL", 13), "ILOVECRIPTLY")
    });
});