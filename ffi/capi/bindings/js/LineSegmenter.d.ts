// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { LineBreakIteratorUtf16 } from "./LineBreakIteratorUtf16"
import type { LineBreakOptions } from "./LineBreakOptions"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X line-break segmenter, capable of finding breakpoints in strings.
*
*See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html) for more information.
*/
export class LineSegmenter {
    

    get ffiValue(): pointer;

    static createAuto(provider: DataProvider): LineSegmenter;

    static createLstm(provider: DataProvider): LineSegmenter;

    static createDictionary(provider: DataProvider): LineSegmenter;

    static autoWithOptions(provider: DataProvider, contentLocale: Locale, options: LineBreakOptions): LineSegmenter;

    static lstmWithOptions(provider: DataProvider, contentLocale: Locale, options: LineBreakOptions): LineSegmenter;

    static dictionaryWithOptions(provider: DataProvider, contentLocale: Locale, options: LineBreakOptions): LineSegmenter;

    segment(input: string): LineBreakIteratorUtf16;
}