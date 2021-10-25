/* tslint:disable */
/* eslint-disable */
/**
*/
export class JsInterpreter {
  free(): void;
/**
* @param {Uint8Array} rom
*/
  constructor(rom: Uint8Array);
/**
* @param {Uint8Array} keypad
*/
  update_keypad(keypad: Uint8Array): void;
/**
*/
  step(): void;
/**
* @returns {boolean}
*/
  should_redraw(): boolean;
/**
* @returns {boolean}
*/
  should_beep(): boolean;
/**
*/
  update_timers(): void;
/**
* @returns {number}
*/
  get_vram_ptr(): number;
/**
* @returns {number}
*/
  get_ram_ptr(): number;
/**
* @returns {number}
*/
  get_pc(): number;
/**
* @returns {number}
*/
  get_v_ptr(): number;
/**
* @returns {number}
*/
  get_i(): number;
/**
* @returns {number}
*/
  get_sp(): number;
/**
* @returns {number}
*/
  get_dt(): number;
/**
* @returns {number}
*/
  get_st(): number;
/**
*/
  reset(): void;
}
