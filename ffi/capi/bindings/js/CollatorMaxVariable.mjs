// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `MaxVariable`](https://docs.rs/icu/latest/icu/collator/enum.MaxVariable.html) for more information.
*/
export class CollatorMaxVariable {
    #value = undefined;

    static #values = new Map([
        ["Space", 0],
        ["Punctuation", 1],
        ["Symbol", 2],
        ["Currency", 3]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return CollatorMaxVariable.#objectValues[arguments[1]];
        }

        if (value instanceof CollatorMaxVariable) {
            return value;
        }

        let intVal = CollatorMaxVariable.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return CollatorMaxVariable.#objectValues[intVal];
        }

        throw TypeError(value + " is not a CollatorMaxVariable and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...CollatorMaxVariable.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new CollatorMaxVariable(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new CollatorMaxVariable(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new CollatorMaxVariable(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new CollatorMaxVariable(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
    ];

    static Space = CollatorMaxVariable.#objectValues[0];
    static Punctuation = CollatorMaxVariable.#objectValues[1];
    static Symbol = CollatorMaxVariable.#objectValues[2];
    static Currency = CollatorMaxVariable.#objectValues[3];
}