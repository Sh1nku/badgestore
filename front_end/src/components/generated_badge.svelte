<script lang="ts">
    import {env} from '$env/dynamic/public';
    import type {Writable} from "svelte/store";
    import type BadgeAuth from "../models/BadgeAuth";
    import {getContext} from "svelte";
    import GeneratedPreview from "../components/generated_preview.svelte";
    import KeyPreview from "../components/key_preview.svelte";
    import type CreateBadge from "../models/CreateBadge";

    let badge_auth:  Writable<BadgeAuth|null> = getContext("badge_auth");
    let badge_form: Writable<CreateBadge> = getContext("badge_form");

    let shields_url: string;
    let badgen_url: string;
    let local_url: string;

    function generate_url(auth: BadgeAuth|null, form: CreateBadge) {
        shields_url = `https://img.shields.io/endpoint?url=${env.PUBLIC_API_PATH}/badge/${auth?.read_key}/shields&style=${form.style}`;
        badgen_url = `https://badgen.net/${env.PUBLIC_API_PATH.replace('://', '/')}/badge/${auth?.read_key}/badgen`;
        local_url = `${env.PUBLIC_API_PATH}/badge/${auth?.read_key}/local?style=${form.style}`;
    }

    badge_form.subscribe((x) => {
        generate_url($badge_auth, x)
    });

    badge_auth.subscribe((x) => {
        generate_url(x, $badge_form)
    });
</script>

{#if $badge_auth !== null}
    <div class="w-full">
        <div class="block uppercase tracking-wide text-gray-700 text-l font-bold mb-2">Generated badge auth</div>
        <KeyPreview name="Read key" value={$badge_auth.read_key}/>
        <KeyPreview name="Write key" value={$badge_auth.write_key}/>
        <div class="block uppercase tracking-wide text-gray-700 text-l font-bold mb-2">Badge URLs</div>
        <div class="flex flex-wrap -mx-3">
            <GeneratedPreview label="Badgestore.dev" url={local_url}/>
            <GeneratedPreview label="Shields.io" url={shields_url}/>
            <GeneratedPreview label="Badgen.net" url={badgen_url}/>
        </div>
    </div>
{/if}