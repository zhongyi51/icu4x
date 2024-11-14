// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `NeoSkeletonLength`](https://docs.rs/icu/latest/icu/datetime/options/enum.NeoSkeletonLength.html) for more information.
*/
export class DateTimeLength {
    #value = undefined;

    static #values = new Map([
        ["Long", 0],
        ["Medium", 1],
        ["Short", 2]
    ]);

    static getAllEntries() {
        return DateTimeLength.#values.entries();
    }

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return DateTimeLength.#objectValues[arguments[1]];
        }

        if (value instanceof DateTimeLength) {
            return value;
        }

        let intVal = DateTimeLength.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return DateTimeLength.#objectValues[intVal];
        }

        throw TypeError(value + " is not a DateTimeLength and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...DateTimeLength.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new DateTimeLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new DateTimeLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new DateTimeLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
    ];

    static Long = DateTimeLength.#objectValues[0];
    static Medium = DateTimeLength.#objectValues[1];
    static Short = DateTimeLength.#objectValues[2];
}