<script lang="ts">
    import {env} from '$env/dynamic/public';
    import Input from './input.svelte';
    import PreviewBadge from "./preview_badge.svelte";
    import { getContext } from "svelte";
    import type {Writable} from "svelte/store";
    import type BadgeAuth from "../models/BadgeAuth";
    import type CreateBadge from "../models/CreateBadge";
    let badge_form: Writable<CreateBadge> = getContext("badge_form");
    let badge_auth:  Writable<BadgeAuth|null> = getContext("badge_auth");

    let creating_badge = false
    let errors = {};

    async function handleForm(e) {

        fetch(env.PUBLIC_API_PATH + '/badge', {
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify($badge_form),
            method: 'POST'
        }).then((response) => {
            switch (response.status) {
                case 201:
                    creating_badge = true;
                    errors = {}
                    response.json().then((json) => {
                        badge_auth.set({
                            read_key: json.read_key,
                            write_key: json.write_key
                        });
                    })
                    break;
                case 400:
                    response.json().then((json) => {
                        errors = json;
                    }).catch(() => {
                        throw new Error();
                    })
                    break;
                case 429:
                    errors = {
                        'global': 'Too many requests'
                    }
                    break;
                default:
                    throw new Error();
            }
            creating_badge = false;
        }).catch(() => {
            errors['global'] = 'An error occurred';
        })
    }
</script>

<form on:submit|preventDefault={handleForm} class="w-full max-w-lg">
    <div class="block uppercase tracking-wide text-gray-700 text-l font-bold mb-2">Create badge</div>
    <div class="flex flex-wrap -mx-3 mb-6">
        <Input
            label="Left Label"
            error={errors['left_label'] !== undefined ? errors['left_label'] : null}
            bind:value={$badge_form.left_label}
        />
        <Input
            label="Right Label"
            error={errors['right_label'] !== undefined ? errors['right_label'] : null}
            bind:value={$badge_form.right_label}
        />
    </div>
    <div class="flex flex-wrap -mx-3 mb-6">
        <Input
               label="Left Color"
               error={errors['left_color'] !== undefined ? errors['left_color'] : null}
               bind:value={$badge_form.left_color}
               type="color"
        />
        <Input
                label="Right Color"
                error={errors['right_color'] !== undefined ? errors['right_color'] : null}
                bind:value={$badge_form.right_color}
                type="color"
        />
    </div>
    <PreviewBadge/>
    <div class="flex flex-wrap -mx-3 mb-6">
        <div class="w-full px-3">
            <button class="w-full bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded" type="submit">
                {#if creating_badge}
                    <div class="animate-spin">|</div>
                {:else }
                    <div class="">Generate</div>
                {/if}
            </button>
            {#if errors['global'] !== undefined}
                <p class="text-red-600">{errors['global']}</p>
            {/if}
        </div>
    </div>
</form>