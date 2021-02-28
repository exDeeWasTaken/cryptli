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
    vigenere_decrypt,
    atbash_encrypt,
    atbash_decrypt,
    latin_encrypt,
    scytale_encrypt,
    scytale_decrypt,
    rotation_encrypt,
    rotation_decrypt
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

Deno.test("Atbash encode", () => {
    const cipher = atbash_encrypt("ILOVECRIPTLY");
    assertEquals(cipher, "ROLEVXIRKGOB");
});

Deno.test("Atbash decode", () => {
    const cipher = atbash_decrypt("ROLEVXIRKGOB");
    assertEquals(cipher, "ILOVECRIPTLY");
});

Deno.test("Latin encode", () => {
    const cipher = latin_encrypt("I love cryptli");
    assertEquals(cipher, "9   12 15 22 5   3 18 25 16 20 12 9");
});

Deno.test("Scytale encode", () => {
    const cipher = scytale_encrypt("abcdefghijklmnopqrstuvwxyzBumBumBum", 3);
    assertEquals(cipher, "adgjmpsvyuuubehknqtwzmmmcfiloruxBBB");
});

Deno.test("Scytale decode", () => {
    const cipher = scytale_decrypt("adgjmpsvyuuubehknqtwzmmmcfiloruxBBB", 3);
    assertEquals(cipher, "abcdefghijklmnopqrstuvwxyzBumBumBum");
});

Deno.test("Rotation encode", () => {
    const cipher = rotation_encrypt("ILOVECRIPTLY", 13);
    assertEquals(cipher, "VYBIRPEVCGYL");
});

Deno.test("Rotation decode", () => {
    const cipher = rotation_decrypt("VYBIRPEVCGYL", 13);
    assertEquals(cipher, "ILOVECRIPTLY");
});