/* tslint:disable */
/* eslint-disable */
/**
* @param {string} str
* @returns {string}
*/
export function morse_encrypt(str: string): string;
/**
* @param {string} str
* @returns {string}
*/
export function morse_decrypt(str: string): string;
/**
* @param {string} str
* @param {number} radix
* @returns {string}
*/
export function numeral_encrypt(str: string, radix: number): string;
/**
* @param {string} str
* @param {number} from_radix
* @returns {string}
*/
export function numeral_decrypt(str: string, from_radix: number): string;
/**
* @param {string} str
* @returns {string}
*/
export function binary_encrypt(str: string): string;
/**
* @param {string} str
* @returns {string}
*/
export function binary_decrypt(str: string): string;
/**
* @param {string} str
* @returns {string}
*/
export function ascii_encrypt(str: string): string;
/**
* @param {string} str
* @returns {string}
*/
export function ascii_decrypt(str: string): string;
/**
* @param {string} plain
* @param {string} key
* @returns {string}
*/
export function vigenere_encrypt(plain: string, key: string): string;
/**
* @param {string} cipher
* @param {string} key
* @returns {string}
*/
export function vigenere_decrypt(cipher: string, key: string): string;
/**
* @param {string} str
* @param {number} shift_amount
* @returns {string}
*/
export function cesar(str: string, shift_amount: number): string;
/**
* @param {string} plain
* @returns {string}
*/
export function atbash_encrypt(plain: string): string;
/**
* @param {string} cipher
* @returns {string}
*/
export function atbash_decrypt(cipher: string): string;
