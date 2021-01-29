import { Scope } from "../tree/Scope";

export class LineContext {
    constructor(
        readonly value: string,
        readonly lineNumber: number,
        readonly scope: Scope
    ) {}
}
