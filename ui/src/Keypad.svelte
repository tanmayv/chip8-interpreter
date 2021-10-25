<script>
import { createEventDispatcher } from 'svelte';
export let useQwerty = false;
const dispatch = createEventDispatcher();
const cb = (event, key) => {dispatch(event, {key}); console.log(event, key)}
$: keymap = !useQwerty ? [
    {key: '1', label: '1'},
    {key: '2', label: '2'},
    {key: '3', label: '3'},
    {key: '4', label: 'C'},
    {key: '\'', label: '4'},
    {key: ',', label: '5'},
    {key: '.', label: '6'},
    {key: 'p', label: 'D'},
    {key: 'a', label: '7'},
    {key: 'o', label: '8'},
    {key: 'e', label: '9'},
    {key: 'u', label: 'E'},
    {key: ';', label: 'A'},
    {key: 'q', label: '0'},
    {key: 'j', label: 'B'},
    {key: 'k', label: 'F'},
]: [
    {key: '1', label: '1'},
    {key: '2', label: '2'},
    {key: '3', label: '3'},
    {key: '4', label: 'C'},
    {key: 'q', label: '4'},
    {key: 'w', label: '5'},
    {key: 'e', label: '6'},
    {key: 'r', label: 'D'},
    {key: 'a', label: '7'},
    {key: 's', label: '8'},
    {key: 'd', label: '9'},
    {key: 'f', label: 'E'},
    {key: 'z', label: 'A'},
    {key: 'x', label: '0'},
    {key: 'c', label: 'B'},
    {key: 'v', label: 'F'},
]
</script>
<div>
    <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">Keypad</div>
    <div class="grid grid-cols-4 gap-2">
        {#each keymap as {key, label}}
            <button 
                on:touchstart={() => cb('down',key)} on:touchend={() => cb('up', key)} 
                on:mousedown={() => cb('down',key)} on:mouseup={() => cb('up', key)} 
                class="hover:bg-accent hover:text-white border-accent border-4 border text-center py-2 uppercase">{label} [{key}]</button>
        {/each}
    </div>
</div>
