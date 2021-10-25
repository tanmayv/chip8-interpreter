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
import GameDescription from "./GameDescription.svelte";
import RegisterViewer from "./RegisterViewer.svelte";
import Button from "../Button.svelte";

let emulator;
let canvas;
let display;
let audio;
let debugInfo;
let playPauseLabel = "pause";
let PLAY = "▶";
let PAUSE = "⏸️";
let romName = "";
let loadingRom = false;
let theme = "accent";
let opts = {
    speed: 10,
    isQwerty: true
};


const initEmulator  = (rom) => {
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
};

const changeRom = async (rom) => {
    console.log("Load rom", rom);
    romName = rom;
    const romBuffer = await (await fetch(`./${romName}`)).arrayBuffer();
    const newRom = new Uint8Array(romBuffer);
    !emulator && initEmulator(newRom);
    loadingRom = true;
    emulator?.pause();
    loadingRom = false;
    emulator?.initMem(newRom)
    emulator.reset();
    emulator.unpause();
};

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
window.addEventListener("keyup", ({key}) => keyup(key));
</script>

<main class={`container scrollbar-thumb-accent gap-2 m-auto bg-black flex flex-col text-${theme} font-bold`}>
    <div>
	    <h2 class="my-8 text-4xl text-center">Chip8 interpreter</h2>
	    <h1 class="my-8 text-lg text-center">A WebAssembly CHIP-8 interpreter written in rust, with svelte frontend.
	    Select a game to continue.</h1>
    </div>
    <GameSelector rom={romName} theme={theme} on:change={async ({detail}) => await changeRom(detail.rom)}></GameSelector>
    <div class="lg:hidden"><GameDescription rom={romName}></GameDescription></div>
	<div class="grid grid-cols-1 w-full gap-4 lg:grid-cols-4">
        <div id="Game" class="w-full flex flex-col gap-4 lg:col-span-3">
            <canvas

	            class="border border-accent p-2 border-4 w-full"
	            bind:this={canvas}
	            width={emulator?.WIDTH}
	            height={emulator?.HEIGHT}></canvas>
	        {#if !loadingRom}
                <div>
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
	        <div class={`flex flex-col gap-2 lg:block`} class:hidden={!emulator}>
	            <Keypad useQwerty={opts.isQwerty} on:down={(e) => keydown(e.detail.key)} on:up={(e) => keyup(e.detail.key)}></Keypad>
                <div class="hidden lg:block"><GameDescription rom={romName}></GameDescription></div>
	        </div>
	    {#if !!emulator}
	        <div class="lg:col-span-2 grid grid-cols-2 gap-2">
	            <RegisterViewer {...debugInfo}></RegisterViewer>
	            <Instructions class="w-full" pc={debugInfo?.pc} instructions={debugInfo?.ram && disassemble(debugInfo?.ram)}></Instructions>
	        </div>
	        <div class="lg:col-span-2">
	            <MemoryViewer theme={theme} ram={debugInfo?.ram}></MemoryViewer>
	        </div>
	    {/if}
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
