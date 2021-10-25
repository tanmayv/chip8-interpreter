<script>
import { onMount } from 'svelte';
import { EmulatorController } from "./chip8";
import { createAudio } from "./chip8/audio";
import { createDisplay } from "./chip8/display";
import { disassemble } from "./chip8/disassembler";
import Instructions from "./Instructions.svelte";
import MemoryViewer from "./MemoryViewer.svelte";
import Keypad from "./Keypad.svelte";
import LayoutSelector from "./LayoutSelector.svelte";
import SpeedSelector from "./SpeedSelector.svelte";
import GameSelector from "./GameSelector.svelte";
import Button from "../Button.svelte";

export let name;
let emulator;
let canvas;
let display;
let audio;
let debugInfo;
let playPauseLabel = "pause";
let PLAY = "▶";
let PAUSE = "⏸️";
let romName = "breakout.ch8";
let loadingRom = false;
let theme = "accent";
let themeSize = 3;
let opts = {
    speed: 10,
    isQwerty: true
}

onMount(async () => {
    loadingRom = true;
    const romBuffer = await (await fetch(`./${romName}`)).arrayBuffer();
    const rom = new Uint8Array(romBuffer);
    loadingRom = false;
    display = createDisplay(
        canvas,
        64,
        32,
        screen.width,
        screen.height
    );
    audio = createAudio();
    emulator = new EmulatorController(rom , opts);
    emulator.onDraw = (vram) => display.draw(vram);
    emulator.onSoundStart = () => audio.start();
    emulator.onSoundStop = () => audio.stop();
    emulator.onUpdate = (info) => debugInfo = info;
});

const changeRom = async (rom) => {
    console.log("Load rom", rom);
    loadingRom = true;
    emulator.pause();
    romName = rom;
    const romBuffer = await (await fetch(`./${romName}`)).arrayBuffer();
    const newRom = new Uint8Array(romBuffer);
    loadingRom = false;
    emulator?.initMem(newRom)
    emulator.reset();
    emulator.unpause();
}

const handlePlayPauseClick = () => {
    if (emulator?.isPaused) {
        playPauseLabel = PAUSE;
        emulator?.unpause();
    } else {
        emulator?.pause();
        playPauseLabel = PLAY;
    }
}

const handleStepClick = () => {
        emulator?.pause();
        playPauseLabel = PLAY;
        emulator?.step();
}

const updateOpts = (opt) => {
    opts = {...opts, ...opt};
    emulator?.updateOpts(opts);
}

const keydown = (key) => emulator?.keyDown(key)
const keyup = (key) => emulator?.keyUp(key)
window.addEventListener("keydown", ({key}) => keydown(key));
window.addEventListener("keydown", ({key}) => {
    if (key == "c") {
        console.log({image: canvas.toDataURL(), value: romName});
    }
});
window.addEventListener("keyup", ({key}) => keyup(key));
</script>

<main class={`container scrollbar-thumb-accent m-auto bg-black flex flex-col text-${theme} font-bold`}>
	<h1 class="my-8 text-2xl text-center">Chip8 interpreter</h1>
	<div class="w-full">
        <canvas
	        class="border border-accent p-2 border-4 w-full"
	        bind:this={canvas}
	        width={emulator?.WIDTH}
	        height={emulator?.HEIGHT}></canvas>
	    {#if !loadingRom}
            <div>
	            <div>Debug controls</div>
	            <div class="grid grid-cols-3 gap-4">
	                <Button text="⟳" on:click={() => emulator?.reset()} isActive={false}></Button>
	                <Button text={playPauseLabel} on:click={handlePlayPauseClick} isActive={false}></Button>
	                <Button text="Step" on:click={handleStepClick} isActive={false}></Button>
	                <LayoutSelector on:click={({detail}) => updateOpts({isQwerty: detail.useQwerty})} useQwerty={opts.isQwerty}></LayoutSelector>
	                <div class="col-span-2">
	                    <SpeedSelector value={opts.speed} on:change={({detail}) => updateOpts({speed: detail.value})}></SpeedSelector>
	                </div>
	            </div>
	        </div>
	    {:else}
	        <div>Loading game</div>
	    {/if}
	</div>

	<div class="flex flex-col gap-4">
	    <Keypad useQwerty={opts.isQwerty} on:down={(e) => keydown(e.detail.key)} on:up={(e) => keyup(e.detail.key)}></Keypad>
	    <Instructions class="w-full" pc={debugInfo?.pc} instructions={debugInfo?.ram && disassemble(debugInfo?.ram)}></Instructions>
        <GameSelector rom={romName} theme={theme} on:change={async ({detail}) => await changeRom(detail.rom)}></GameSelector>
	    <MemoryViewer theme={theme} ram={debugInfo?.ram}></MemoryViewer>
	</div>
</main>

<style global>
@tailwind base;
@tailwind components;
@tailwind utilities;
main {
    font-family: 'VT323', monospace;
}

h1 {
	text-transform: uppercase;
	font-size: 4em;
	font-weight: 100;
}

@media (min-width: 640px) {
	main {
		max-width: none;
	}
}
</style>
