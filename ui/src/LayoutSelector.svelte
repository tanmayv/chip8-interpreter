<script>
import { createEventDispatcher } from 'svelte';
export let useQwerty = false;
const dispatch = createEventDispatcher();

</script>
<div class="grid grid-cols-2 border border-accent rounded">
    
    <button on:click={() => dispatch('click', {useQwerty: true})} class="border-none" class:bg-accent={useQwerty} class:text-white={useQwerty}>Qwerty</button>
    <button on:click={() => dispatch('click', {useQwerty: false})} class="border-none"class:bg-accent={!useQwerty} class:text-white={!useQwerty}>Dvorak</button>
</div>
