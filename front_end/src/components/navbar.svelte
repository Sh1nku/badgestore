<script lang="ts">
    import {env} from '$env/dynamic/public';
</script>

<nav class="flex items-center justify-between flex-shrink-0 p-6">
    <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
        <div class="text-sm lg:flex-grow">
            <a href='/disclaimers' class="font-mono inline-block mt-0 pr-4 text-black hover:text-gray-600">
                Disclaimers
            </a>
            <a href={env.PUBLIC_API_PATH} class="font-mono inline-block mt-0 pr-4 text-black hover:text-gray-600">
                API
            </a>
            <div class="inline-block mt-0 p-4" >|</div>
            <a href="https://github.com/Sh1nku/badgestore" class="inline-block mt-0 align-middle">
                <img src="/github-mark.svg" alt="Project github page" class="object-cover h-4"/>
            </a>

        </div>
    </div>
</nav>