<script lang="ts">
    export let label;
    export let error;
    export let value;
    export let type = 'text';
</script>

<div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
    <label for={label} class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">
        {label}
    </label>
    {#if type === 'text'}
        <input id={label} class="block w-full h-12 bg-gray-200 text-gray-700 border border-gray-200 rounded py-1 px-4 " bind:value={value}>
    {:else if type === 'color'}
        <input id={label} class="block w-full h-12 bg-gray-200 text-gray-700 border border-gray-200 rounded py-1 px-4 " type="color" bind:value={value}>
    {/if}
    {#if error !== null && error !== undefined}
        <p class="text-red-600">{error}</p>
    {/if}
</div>