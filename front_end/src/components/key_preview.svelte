<script lang="ts">
    export let name;
    export let value;

    function copyClipboard(button) {
        button.target.innerText = 'Copied!';
        setTimeout(() => {
            button.target.innerText = 'Copy to clipboard';
        }, 500);
        navigator.clipboard.writeText(value);
    }
</script>

<div class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">{name}</div>
<div class="w-full">
    <div class="block h-6 bg-gray-200 text-gray-700 border border-gray-200 rounded px-4">{value}</div>
    <button on:click={copyClipboard} class="inline-block h-6 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border text-white px-2 rounded">
        Copy to clipboard
    </button>
</div>