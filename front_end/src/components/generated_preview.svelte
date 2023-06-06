<script lang="ts">
    export let label;
    export let url;

    function copyClipboard(button) {
        button.target.innerText = 'Copied!';
        setTimeout(() => {
            button.target.innerText = 'Copy to clipboard';
        }, 500);
        navigator.clipboard.writeText(url);
    }
</script>

<div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
    <div class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">{label}</div>
    <img src={url}  alt="{label} generated badge"/>
    <button on:click={copyClipboard} class="mt-2 w-full bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded">
        Copy to clipboard
    </button>
</div>