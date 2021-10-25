<script>
export let text, isActive;
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
const cb = () => dispatch('click')

</script>
<main class="w-full">
    <button class="uppercase px-4 w-full" on:click={cb} class:opacity-75={isActive}>{text}</button>
</main>
<style></style>
