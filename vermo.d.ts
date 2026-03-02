/* tslint:disable */
/* eslint-disable */

export function bevy_arquivo_recebido(bytes: Uint8Array): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly bevy_arquivo_recebido: (a: any) => void;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen__closure__destroy__h54eeb27f1ffb43ea: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h0ca1c2f35881cd48: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h6e9757a276028034: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h54c07ffc8f1b2915: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h1b6ac8bcc09cae6f: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h04860ccc24643a32_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h426d50c271b548b1: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h5f518a2efc93c3b2: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hee1e444265cc4d99: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
