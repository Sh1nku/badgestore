<script lang="ts">
    import "../app.css";
    import Header from "../components/header.svelte";
</script>
<div class="px-4 sm:px-6 md:px-8">
<Header />
</div>
<div class="relative max-w-5xl mx-auto ">
    <slot />
</div>

<style lang="postcss">
    :global(html) {
        background-color: theme(colors.gray.100);
    }
</style>