import { LineContext } from "../structures/LineContext";

export class CompilerError extends SyntaxError {
    constructor(readonly line: LineContext, readonly message: string) {
        super(message);
        this.name = "CompilerError";
        this.stack = `CompilerError: ${this.message}\n\tat line ${line.lineNumber}: ${line.value}`;
    }
}
