import { assertEquals } from "https://deno.land/std@0.84.0/testing/asserts.ts";
import {cesar} from "./mod.ts";

Deno.test("Cesar encode mixed", () => {
    const cipher = cesar("aAzZ!.?`", 50);
    assertEquals(cipher, "yYxX!.?`");
});

Deno.test("Cesar decode mixed", () => {
    const cipher = cesar("yYxX!.?`", -50);
    assertEquals(cipher, "aAzZ!.?`");
});