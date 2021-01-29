import { readFileSync } from "fs";

import { Matcher } from "./Matcher";
import { LineContext } from "./structures/LineContext";
import { Assignment } from "./tree/Assignment";
import { Scope } from "./tree/Scope";

const source = "let x = 0; x += 1";

const input = readFileSync("./test.stx").toString("utf-8");

export const globalScope = new Scope([]);

const tokenParser = new Matcher(input)
    .trim()
    .replaceAll("\n", ";")
    .apply(console.log);
