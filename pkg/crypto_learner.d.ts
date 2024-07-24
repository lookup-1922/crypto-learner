/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function generate_aes_key(): string;
/**
* @param {string} plaintext
* @param {string} key
* @returns {string}
*/
export function encrypt_aes(plaintext: string, key: string): string;
/**
* @param {string} ciphertext
* @param {string} key
* @returns {string}
*/
export function decrypt_aes(ciphertext: string, key: string): string;
/**
* @returns {any}
*/
export function generate_rsa_key(): any;
/**
* @param {string} plaintext
* @param {string} public_key_pem
* @returns {string}
*/
export function encrypt_rsa(plaintext: string, public_key_pem: string): string;
/**
* @param {string} ciphertext
* @param {string} private_key_pem
* @returns {string}
*/
export function decrypt_rsa(ciphertext: string, private_key_pem: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate_aes_key: (a: number) => void;
  readonly encrypt_aes: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly decrypt_aes: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly generate_rsa_key: (a: number) => void;
  readonly encrypt_rsa: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly decrypt_rsa: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
