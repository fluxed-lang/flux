import { readFileSync } from "fs";

import { Matcher } from "./Matcher";
import { Scope } from "./tree/Scope";

const input = readFileSync("./test.stx").toString("utf-8");

export const globalScope = new Scope([]);

const tokenParser = new Matcher(input)
    .trim()
    .replaceAll("\n", ";")
    .apply(console.log);
