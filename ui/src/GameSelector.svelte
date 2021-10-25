<script>
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
export let rom;
export let theme;
const library = [
    {value: "astrododge.ch8", label: "Astrododge", image: "astrododge.png"},
    {value: "breakout.ch8", label: "Breakout", image: "breakout.png"},
    {value: "spaceinvaders.ch8", label: "Space Invaders", image: "spaceinvaders.png"},
    {value: "zero.ch8", label: "Zero", image: "zero.png"},
    {value: "tank.ch8", label: "Tank", image: "tank.png"},
    {value: "kaleidoscope.ch8", label: "Kaleidoscope", image: "kaleidoscope.png"},
    {value: "coinflip.ch8", label: "Coin flip", image: "coinflip.png"},
];
</script>
<div>
    <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">Select a game to continue</div>
    <div class={`scrollbar scrollbar-thin pb-4 scrollbar-thumb-${theme} w-full flex gap-1 overflow-x-auto cursor-move`}>

        {#each library as {value, label, image}}
            <div class:selected={rom === value} class="w-48 relative border-1 p-2 border-accent flex-grow-0 flex-shrink-0" on:click={() => dispatch('change', {rom: value})}>
                <img src={image} class="w-full" alt={label}/>
                <div>{label}</div>
                {#if rom === value}
                    <div class="bg-black bg-opacity-60 px-2 right-0 top-0 absolute">NOW PLAYING</div>
                {/if}
            </div>
        {/each}
    </div>
</div>
<style>
.selected { border: 4px solid; }
img {
    image-rendering: pixelated;
}
</style>
