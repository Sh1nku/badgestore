<script lang="ts">
    import Li from '../../components/li.svelte'
</script>

<svelte:head>
    <link rel="canonical" href="https://badgestore.dev/disclaimers" />
</svelte:head>

<h1 class="font-mono font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center">
    Disclamers
</h1>
<div class="flex flex-wrap flex-row justify-center pt-8">
    <div class="p-4">
        <ul class="ml-10">
            <Li>There is no permanent identifying information stored on who creates a badge.</Li>
            <Li>Don't store sensitive information. There is no way to delete it.</Li>
            <Li>If you lose the write_key for a badge, there is no way to restore it.</Li>
            <Li>There is a Per-IP limit of 100 badges created per day.</Li>
            <Li>I reserve the right to delete any badge at any time, for any reason.</Li>
            <Li>To do cleanup I will be deleting badges that have not been accessed for 365 days,
                or contain the default example input given by the form on the website or the API</Li>
            <Li>This site is not in any way affiliated with
                <a href="https://shields.io" class="underline text-blue-800">shields.io</a> or
                <a href="https://badgen.net" class="underline text-blue-800">badgen.net</a>.
            </Li>
            <Li>This site uses <a class="underline text-blue-800" href="https://gitlab.com/tangram-vision/oss/rsbadges">rsbadges</a> to generate badges internally, but is not in any way affiliated with the library.</Li>
        </ul>
    </div>
</div>