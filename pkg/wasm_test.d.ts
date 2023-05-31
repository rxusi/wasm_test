/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export class Point {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  constructor(x: number, y: number);
/**
*/
  show(): void;
/**
* @returns {number}
*/
  sum(): number;
/**
* @param {number} x
* @param {number} y
*/
  set(x: number, y: number): void;
}
/**
*/
export class Tictactoe {
  free(): void;
/**
* @param {number} _N
* @param {number} _win_N
* @param {number} first
*/
  constructor(_N: number, _win_N: number, first: number);
/**
* @returns {Tictactoe}
*/
  clone(): Tictactoe;
/**
* @param {number} stone
* @param {number} _y
* @param {number} _x
* @param {string} strategy
* @returns {string}
*/
  put(stone: number, _y: number, _x: number, strategy: string): string;
/**
* @param {number} y
* @param {number} x
* @returns {string}
*/
  getCellStr(y: number, x: number): string;
/**
* @param {number} v
* @returns {string}
*/
  static cell2Str(v: number): string;
/**
*/
  show(): void;
/**
* @returns {string}
*/
  getBoardHTML(): string;
/**
* @returns {number}
*/
  getWinner(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_point_free: (a: number) => void;
  readonly point_new: (a: number, b: number) => number;
  readonly point_show: (a: number) => void;
  readonly point_sum: (a: number) => number;
  readonly point_set: (a: number, b: number, c: number) => void;
  readonly __wbg_tictactoe_free: (a: number) => void;
  readonly tictactoe_new: (a: number, b: number, c: number) => number;
  readonly tictactoe_clone: (a: number) => number;
  readonly tictactoe_put: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly tictactoe_getCellStr: (a: number, b: number, c: number, d: number) => void;
  readonly tictactoe_cell2Str: (a: number, b: number) => void;
  readonly tictactoe_show: (a: number) => void;
  readonly tictactoe_getBoardHTML: (a: number, b: number) => void;
  readonly tictactoe_getWinner: (a: number) => number;
  readonly greet: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
