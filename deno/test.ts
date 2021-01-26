import { assertEquals } from "https://deno.land/std@0.84.0/testing/asserts.ts";
import {cesar} from "./mod.ts";

Deno.test("Cesar encode mixed", () => {
    const cipher = cesar("aAzZ!.?`", 50);
    assertEquals(cipher, "yYxX!.?`");
});

Deno.test("Cesar encode lower", () => {
   const cipher = cesar("z", 1);
   assertEquals(cipher, "a");
});

Deno.test("Cesar decode upper", () => {
    const cipher = cesar("C", -1);
    assertEquals(cipher, "B");
});

Deno.test("Cesar decode lower", () => {
    const cipher = cesar("z", -1);
    assertEquals(cipher, "y");
});