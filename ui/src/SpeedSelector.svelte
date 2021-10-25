<script>
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
export let value = 10;
</script>
<div class="flex gap-2 items-center">
    <label>Speed: </label>
    <input class="w-full rounded-lg overflow-hidden appearance-none border-0 bg-gray-500 h-3" min="0" max="15" value={value} type="range" on:change={(e) => dispatch('change', {value: parseInt(e.target.value)})}>
</div>
<style>
        input[type="range"]::-webkit-slider-thumb {
            width: 15px;
            -webkit-appearance: none;
  			appearance: none;
            height: 15px;
            cursor: ew-resize;
            background: #77D970;
            box-shadow: -405px 0 0 400px #77D970;
            border-radius: 50%;
            
        }


</style>
