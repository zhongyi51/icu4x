// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** See the [Rust documentation for `CaseLevel`](https://docs.rs/icu/latest/icu/collator/enum.CaseLevel.html) for more information.
*/
export class CollatorCaseLevel {
    constructor(value : CollatorCaseLevel | string);

    get value() : string;

    get ffiValue() : number;

    static Off : CollatorCaseLevel;
    static On : CollatorCaseLevel;
}