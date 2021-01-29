import { Variable } from "./Variable";

export class Scope {
    readonly variables = new Map<string, Variable<any>>();
    readonly children = new Set<Scope>();

    constructor(readonly inheritedVars: Variable<any>[]) {}

    public child(factory: (scope: Scope) => Scope) {
        this.children.add(
            factory(new Scope(Array.from(this.variables.values())))
        );
    }

    public createVariable() {}
}
