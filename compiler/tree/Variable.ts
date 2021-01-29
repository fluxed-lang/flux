import { DataType } from "../types/DataType";

export class Variable<T extends DataType> {
    constructor(
        readonly name: string,
        readonly type: T,
        readonly mutable: boolean = false
    ) {}
}
