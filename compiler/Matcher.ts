export class Matcher {
    constructor(public value: string) {}

    match(string: string, callback: (m: Matcher) => Matcher) {
        if (this.value.startsWith(string)) {
            callback(new Matcher(string));
        }
        return this;
    }

    replace(from: string | RegExp, to: string) {
        this.value = this.value.replace(from, to);
        return this;
    }

    replaceAll(from: string | RegExp, to: string) {
        this.value = this.value.replaceAll(from, to);
        return this;
    }

    apply(cb: (val: string) => void) {
        cb(this.value);
        return this;
    }

    modify(cb: (val: string) => string) {
        this.value = cb(this.value);
        return this;
    }

    trim() {
        this.value = this.value.trim();
        return this;
    }
}
