// generated by diplomat-tool
import { BidiInfo } from "./BidiInfo.mjs"
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { ReorderedIndexMap } from "./ReorderedIndexMap.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Bidi object, containing loaded bidi data
*
*See the [Rust documentation for `BidiClassAdapter`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
*/
const Bidi_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_Bidi_destroy_mv1(ptr);
});

export class Bidi {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Bidi is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Bidi_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static create(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_Bidi_create_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new Bidi(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    forText(text, defaultLevel) {
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const textSlice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, text));
        
        // This lifetime edge depends on lifetimes 'text
        let textEdges = [textSlice];
        
        const result = wasm.icu4x_Bidi_for_text_valid_utf8_mv1(this.ffiValue, ...textSlice.splat(), ...diplomatRuntime.optionToArgsForCalling(defaultLevel, 1, 1, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue, Uint8Array)]));
    
        try {
            return new BidiInfo(diplomatRuntime.internalConstructor, result, [], textEdges);
        }
        
        finally {
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        }
    }

    reorderVisual(levels) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const levelsSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.slice(wasm, levels, "u8"));
        
        const result = wasm.icu4x_Bidi_reorder_visual_mv1(this.ffiValue, ...levelsSlice.splat());
    
        try {
            return new ReorderedIndexMap(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    static levelIsRtl(level) {
        const result = wasm.icu4x_Bidi_level_is_rtl_mv1(level);
    
        try {
            return result;
        }
        
        finally {}
    }

    static levelIsLtr(level) {
        const result = wasm.icu4x_Bidi_level_is_ltr_mv1(level);
    
        try {
            return result;
        }
        
        finally {}
    }

    static levelRtl() {
        const result = wasm.icu4x_Bidi_level_rtl_mv1();
    
        try {
            return result;
        }
        
        finally {}
    }

    static levelLtr() {
        const result = wasm.icu4x_Bidi_level_ltr_mv1();
    
        try {
            return result;
        }
        
        finally {}
    }
}