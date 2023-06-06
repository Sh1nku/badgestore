<script lang="ts">
    import type {Writable} from "svelte/store";
    import type CreateBadge from "../models/CreateBadge";
    import {getContext} from "svelte";
    import {env} from '$env/dynamic/public';
    import {BadgeStyle} from "../models/BadgeStyle";

    let badge_form: Writable<CreateBadge> = getContext("badge_form");

    let local_url: string;
    badge_form.subscribe(x => {
        const left_label = encodeURIComponent(x.left_label);
        const right_label = encodeURIComponent(x.right_label);
        const left_color = encodeURIComponent(x.left_color);
        const right_color = encodeURIComponent(x.right_color);

        local_url = `${env.PUBLIC_API_PATH}/badge/preview?left_label=${left_label}&right_label=${right_label}&left_color=${left_color}&right_color=${right_color}&style=${x.style}`;
    });
</script>

<div class="bg-">
    <div class="block uppercase tracking-wide text-gray-700 text-l font-bold mb-2">Preview Badge</div>
    <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <div class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">Preview</div>
                <img src={local_url}  alt="Preview of Badge"/>
            </div>
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <div class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">Style</div>
                <select class="block w-full h-8 bg-gray-200 text-gray-700 border border-gray-200 rounded py-1 px-4" bind:value={$badge_form.style}>
                    {#each Object.values(BadgeStyle) as style}
                        <option value={style}>{style}</option>
                    {/each}
                </select>
            </div>
    </div>
</div>