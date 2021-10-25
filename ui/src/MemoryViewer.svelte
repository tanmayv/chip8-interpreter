<script>
import { onMount } from 'svelte';
import { hexformat } from "./chip8/utils";
export let ram;
export let theme;
let scroller;
const processAddr = (total, current, index) => {
    const last = total.pop();
    let toAdd = last;
    if (!last || last.bytes.length == 16) {
       if (last) {
           total.push(last);
       } 
       toAdd = { addr: hexformat(0x201 + index, 4), bytes: []}
    }
    toAdd.bytes.push(current);
    total.push(toAdd);
    if (index === ram.length - 1) {
        lastRam = JSON.parse(JSON.stringify(ram || []));
    }
    return total;
}
$: processedRam = ram?.slice(0x201, 0x1000).reduce(processAddr, []) || [];

onMount(() => {
    console.log(ram);
})
</script>

<div class=" w-full">
    <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">Memory</div>
    <div class={`scrollbar scrollbar-thin scrollbar-thumb-${theme} container text-left`} bind:this={scroller}>
        {#each processedRam as {addr, bytes}}
            <div class="flex gap-4 selected" id={`current-${addr}`}>
                <div class="instruction" >{hexformat(addr, 4)}</div>
                <div class="instruction flex-grow flex overflow-x-auto" >
                    {#each bytes as byte}
                        <div class="w-8 flex-grow">{byte?.toString(16).padStart(2, "0")} </div>
                    {/each}
                </div>
            </div>
        {/each}
    </div>
</div>
<style>

.container {
    max-height: 280px;
    overflow-y: scroll;
    overflow-x: hidden;
}
</style>
