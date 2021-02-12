import {assertEquals} from "https://deno.land/std@0.84.0/testing/asserts.ts";
import {
    binary_decrypt,
    binary_encrypt,
    cesar,
    numeral_encrypt,
    numeral_decrypt,
    ascii_encrypt,
    ascii_decrypt,
    morse_encrypt, 
    morse_decrypt,
    vigenere_encrypt,
    vigenere_decrypt
} from "./mod.ts";

Deno.test("Cesar encode", () => {
    const cipher = cesar("aAzZ!.?`", 50);
    assertEquals(cipher, "yYxX!.?`");
});

Deno.test("Cesar decode", () => {
    const cipher = cesar("yYxX!.?`", -50);
    assertEquals(cipher, "aAzZ!.?`");
});

Deno.test("Binary encode", () => {
    const cipher = binary_encrypt("I <3 cryptli!");
    assertEquals(cipher, "1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001");
});

Deno.test("Binary decode", () => {
    const cipher = binary_decrypt("1001001 100000 111100 110011 100000 1100011 1110010 1111001 1110000 1110100 1101100 1101001 100001");
    assertEquals(cipher, "I <3 cryptli!");
});

Deno.test("Numeral encode", () => {
    const cipher = numeral_encrypt("I <3 cryptli!", 36);
    assertEquals(cipher, "21 w 1o 1f w 2r 36 3d 34 38 30 2x x");
});

Deno.test("Numeral decode", () => {
    const cipher = numeral_decrypt("21 w 1o 1f w 2r 36 3d 34 38 30 2x x", 36);
    assertEquals(cipher, "I <3 cryptli!");
});

Deno.test("Ascii encode", () => {
    const cipher = ascii_encrypt("I <3 cryptli!");
    assertEquals(cipher, "73 32 60 51 32 99 114 121 112 116 108 105 33");
});

Deno.test("Ascii decode", () => {
    const cipher = ascii_decrypt("73 32 60 51 32 99 114 121 112 116 108 105 33");
    assertEquals(cipher, "I <3 cryptli!");
});

Deno.test("Morse encode", () => {
   const cipher = morse_encrypt("abcD");
   assertEquals(cipher, ".- -... -.-. -..")
});

Deno.test("Morse decode", () => {
   const cipher = morse_decrypt(".- -... -.-. -..");
   assertEquals(cipher, "abcd");
});

Deno.test("Vigenere encode", () => {
    const cipher = vigenere_encrypt("ILOVECRIPTLY", "KEY");
    assertEquals(cipher, "SPMFIABMNDPW");
 });

 Deno.test("Vigenere decode", () => {
    const cipher = vigenere_decrypt("SPMFIABMNDPW", "KEY");
    assertEquals(cipher, "ILOVECRIPTLY");
 });