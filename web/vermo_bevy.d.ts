/* tslint:disable */
/* eslint-disable */

export function bevy_carregar_modelo(bytes: Uint8Array): void;

export function bevy_definir_luz_cor(r: number, g: number, b: number): void;

export function bevy_definir_luz_intensidade(v: number): void;

export function bevy_definir_luz_rotacao(x: number, y: number, z: number): void;

export function bevy_definir_modelo_escala(v: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly bevy_carregar_modelo: (a: any) => void;
    readonly bevy_definir_luz_cor: (a: number, b: number, c: number) => void;
    readonly bevy_definir_luz_intensidade: (a: number) => void;
    readonly bevy_definir_luz_rotacao: (a: number, b: number, c: number) => void;
    readonly bevy_definir_modelo_escala: (a: number) => void;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__h193f663c53dc2979: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__ha623e9a72780c663: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1419c8f7730678f2_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hc7297b135636c2e6: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h320bdbc6cbbed961: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h913777764bca6704: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
