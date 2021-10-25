import * as libchipolata from "libchipolata";
import { memory } from "libchipolata/libchipolata_bg.wasm";

import { createAudio } from "./audio";
import { createDisplay } from "./display";
import { disassemble } from "./disassembler";
import { makeKeypad } from "./keypad";
import { hexformat } from "./utils";

const defaultOpts = {
    speed: 10,
    isQwerty: false
}

export class EmulatorController {
    
    constructor(rom, opts) {
        this.WIDTH = 64;
        this.HEIGHT = 32;

        this.opts = {...defaultOpts, ...opts };
        this.isPaused = false;
        this.isMuted = false;
        this.interpreter = null;
        this.vram = null;
        this.ram = null;
        this.isKeyPressed = {};
        this.onDraw = () => {};
        this.onSoundStart = () => {};
        this.onSoundStop = () => {};
        this.onUpdate = () => {};
        this.initMem(rom);
        this.start();
    }

    initMem(rom) {
        this.interpreter = new libchipolata.JsInterpreter(rom);
        this.vRegisters = new Uint8Array(
            memory.buffer,
            this.interpreter.get_v_ptr(),
            16
        );
        this.vram = new Uint8Array(
            memory.buffer,
            this.interpreter.get_vram_ptr(),
            this.WIDTH * this.HEIGHT
        );
        this.ram = new Uint8Array(
            memory.buffer,
            this.interpreter.get_ram_ptr(),
            0x1000
        );
    }

    start() {
        const renderLoop = () => {
            if (!this.isPaused) {
                this.tick(this.opts.speed)
            }

            requestAnimationFrame(renderLoop);
        };

        renderLoop();
    }

    keyDown(code) {
        this.isKeyPressed[code] = true;
    }

    keyUp(code) {
        this.isKeyPressed[code] = false;
    }

    pause() {
        this.isPaused = true;
    }

    unpause() {
        this.isPaused = false;
    }

    step() {
        this.pause();
        this.tick(1);
    }

    updateOpts(opts) {
        this.opts = opts;
        console.log("Opts are updated", this.opts);
    }

    tick(count = 1) {
                let redraw = false;
                for (let i = 0; i < count; i++) {
                    this.interpreter.update_keypad(makeKeypad(this.isKeyPressed, this.opts.isQwerty));
                    this.interpreter.step();

                    if (this.interpreter.should_redraw()) {
                        redraw = true;
                    }
                }

                if (redraw) {
                    this.onDraw(this.vram);
                    // this.display.draw(this.vram);
                }

                if (!this.isMuted) {
                    if (this.interpreter.should_beep()) {
                        // this.audio.start();
                        this.onSoundStart();
                    } else {
                        this.onSoundStop();
                        // this.audio.stop();
                    }
                }

            this.interpreter.update_timers();
            this.onUpdate(this.debugInfo());
        }

    reset () {
        this.interpreter.reset();
    }

    debugInfo() {
        return {
            pc: this.interpreter.get_pc(),
            i: this.interpreter.get_i(),
            sp: this.interpreter.get_sp(),
            dt: this.interpreter.get_dt(),
            st: this.interpreter.get_st(),
            registers: this.vRegisters,
            ram: this.ram
        }
    }
}
