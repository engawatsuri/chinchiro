/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const draw_map: () => [number, number];
export const move_train: (a: number, b: number) => [number, number];
export const stop_blue_station: () => bigint;
export const stop_card_station: () => [number, number];
export const stop_property_station: () => [number, number];
export const stop_red_station: () => bigint;
export const add_player: (a: number, b: number) => void;
export const get_cards: () => [number, number];
export const next_turn: () => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __externref_table_dealloc: (a: number) => void;
export const __externref_drop_slice: (a: number, b: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
