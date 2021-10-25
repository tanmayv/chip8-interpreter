<script>
export let pc, registers = [], i, st, dt, sp;
import { hexformat } from "./chip8/utils";
$: values = [
    {'label': 'counter', value: pc},
    {'label': 'index', value: i},
    {'label': 'sound_timer', value: st},
    {'label': 'delay_timer', value: dt},
];
</script>

<div class="w-full">
    <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">Registers</div>
    <div class="grid grid-cols-2">
        {#each values as {label, value}}
            <div>{label}: {hexformat(value || 0)}</div>
        {/each}
        {#each registers as r, idx}
            <div>{`v[${idx}]`}: {hexformat(r || 0)}</div>
        {/each}
    </div>   
</div>
