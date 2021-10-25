<script>
import { hexformat } from "./chip8/utils";
let clazz;
export { clazz as class };
export let instructions = [];
export let pc = 0;
let scroller;
$: if (pc > 0 && scroller) {
    const current = scroller.querySelector(`#current-${pc}`) || scroller.querySelector(`#current-${pc + 1}`);
    current && scroller.scrollTo(0, current.offsetTop - scroller.offsetTop - 50);
}
</script>
<main class={clazz || ''}>
    <div class="w-full">
        <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">Instructions</div>
        <div class="container text-left" bind:this={scroller}>
            {#each instructions as {addr, instruction}}
                <div id={`current-${addr}`} class="{addr == pc || addr == pc+1 ? 'flex gap-4 bg-accent text-white' : 'flex gap-4'}" >
                    <div class="instruction" >{hexformat(addr, 4)}</div>
                    <div class="instruction" >{instruction}</div>
                </div>
            {/each}
        </div>
    </div>
</main>

<style>
.container {
    max-height: 280px;
    overflow-y: hidden;
}
</style>
