import { LineContext } from "../structures/LineContext";
import { DataType } from "../types/DataType";
import { CompilerError } from "../util/errors";
import { Expression } from "./Expression";
import { Scope } from "./Scope";
import { Variable } from "./Variable";

enum AssignmentType {
    EQUALS,
    PLUS_EQUALS,
    MINUS_EQUALS,
    MULT_EQUALS,
    DIV_EQUALS,
    INCREMENT,
    DECREMENT,
}

export class Assignment<T extends DataType> extends Expression<T> {
    static ASSIGNMENT_REGEX = /(?<declaration>let )?(?<mutable>mut )?(?<tokens>[A-z, ]+\b) ?(?<assignmentType>[-+*\/]?=) ?(?<value>[0-9](?=;|$))/;

    constructor(
        readonly variable: Variable<T>,
        readonly operation: AssignmentType
    ) {
        super();
    }

    static createAssignment(line: LineContext) {
        const { groups } = line.value.match(Assignment.ASSIGNMENT_REGEX);

        const isDeclaration = !!groups.declaration;
        const isMutable = !!groups.mutable;
        const tokens = groups.tokens.split(", ");

        tokens.forEach((t) => {
            if (!line.scope.variables.has(t) && !isDeclaration) {
                throw new CompilerError(
                    line,
                    "cannot assign an undeclared variable"
                );
            }

            if (!line.scope.variables.has(t) && isDeclaration) {
                line.scope.variables.push(new Variable(t, ));
            }

            if (line.scope.variables.has(t) && isDeclaration) {
                throw new CompilerError(
                    line,
                    "cannot reassign a declared variable"
                );
            }
        });
    }

    static extractDataType(typeOrValue: string); {}

    static isAssignment(line: LineContext) {
        return Assignment.ASSIGNMENT_REGEX.test(line.value);
    }
}
