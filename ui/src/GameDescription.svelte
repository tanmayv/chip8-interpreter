<script>
export let rom;
console.log(rom);
</script>
<div class="w-full">
    {#if rom ==="breakout.ch8"}
        <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">How to play</div>
        <div>Press <span class="text-accent2">Q</span> and <span class="text-accent2">E</span> to move the paddle left and right. You have 5 lives shown on top right of the screen</div>
    {/if}
    {#if rom ==="spaceinvaders.ch8"}
        <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">How to play</div>
            <div>Press <span class="text-accent2">W</span> to start. Press <span class="text-accent2">Q</span> and <span class="text-accent2">E</span> to move left and right. 
            <span class="text-accent2">W</span> to shoot.</div>
    {/if}
    {#if rom ==="astrododge.ch8"}
        <div class="border-b-2 border-dashed border-accent text-xl p-2 mb-1">How to play</div>
            <div>Press <span class="text-accent2">W</span> to start. Press <span class="text-accent2">Q</span> and <span class="text-accent2">E</span> to move left and right. 
            <span class="text-accent2">2</span> and <span class="text-accent2">S</span> to move up and down.</div>
            <div>Avoid the astroids to win.</div>
    {/if}
</div>
