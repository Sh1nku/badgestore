<script lang="ts">
    import BadgeForm from "../components/badge_form.svelte"
    import GeneratedBadge from "../components/generated_badge.svelte"
    import Li from "../components/li.svelte"
    import {env} from "$env/dynamic/public";

    import type BadgeAuth from "../models/BadgeAuth";
    import {BadgeStyle} from "../models/BadgeStyle";
    import type CreateBadge from "../models/CreateBadge";
    import type {Writable} from "svelte/store";
    import {writable} from "svelte/store";
    import {setContext} from "svelte";

    const s_badge_auth: Writable<BadgeAuth|null> = writable(null);
    const s_badge_form: Writable<CreateBadge> = writable({
        left_label: 'Lines of code',
        right_label: '-100',
        left_color: '#555555',
        right_color: '#999999',
        style: BadgeStyle.flat,
    });
    setContext("badge_form", s_badge_form);
    setContext("badge_auth", s_badge_auth);
</script>

<svelte:head>
    <link rel="canonical" href="https://badgestore.dev/" />
</svelte:head>

<h1 class="font-mono font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center">
    Store and update the contents of your badges
</h1>
<div class="flex flex-wrap flex-row justify-center pt-8">
    <div class="p-4">
        <ul class="ml-10">
            <Li class="mb-2 text-2xl sm:text-2xl lg:text-3xl text-left">No registration!</Li>
            <Li class="mb-2 text-2xl sm:text-2xl lg:text-3xl text-left">Support for generating
                <a href="https://shields.io" class="underline text-blue-800">shields.io</a> and
                <a href="https://badgen.net" class="underline text-blue-800">badgen.net</a> badges
            </Li>
            <Li class="mb-2 text-2xl sm:text-2xl lg:text-3xl text-left">Update using
                <a href={env.PUBLIC_API_PATH} class="underline text-blue-800">API</a> or <a class="underline text-blue-800" href="https://github.com/Sh1nku/badgestore-update-badge-action">Github Action</a>
            </Li>
        </ul>
    </div>
    <div class="p-4 basis-full">
        <div class="m-auto w-fit">
            <BadgeForm />
            <GeneratedBadge />
        </div>
    </div>
</div>