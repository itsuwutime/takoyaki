<script setup lang="ts">
import { useMeta } from "vue-meta";

import Plus from "../components/Plus.vue";
import Feature from "../components/Feature.vue";

useMeta({
    title: "Blazingly fast git contributions in your terminal"
})
</script>

<template>
    <div
        class="w-screen h-screen bg-base px-20 flex items-center justify-center"
    >
        <div class="text-text mr-auto pr-10 pl-14 w-1/2">
            <div class="text-5xl font-bold leading-[60px]">
                Get your git contributions in your terminal.
            </div>
            <div class="font-semibold mt-4 text-overlay1 tracking-wide">
                Takoyaki is an blazingly fast, open source tool that prints your git contributions in your terminal. Not only it allows you to customize it, but also install several plugins to improve overall experience [Fun fact: It can be even used to make some other things!]
            </div>
            <div class="features mt-8 flex flex-col gap-y-5">
                <Feature title="Open source" />
                <Feature title="Blazingly fast" />
                <Feature title="Highly customizable" />
                <Feature title="Cross platform support" />
                <Feature title="Plugins that support popular providers" />
            </div>
            <a href="/install">
                <button
                    class="mt-10 bg-surface0/40 p-4 px-10 rounded-lg text-overlay3 font-semibold"
                >
                    Behold the power of takoyaki
                </button>
            </a>
        </div>
        <div class="banner w-5/12 relative ml-10">
            <img src="/banner.png" class="w-full" />
            <div
                class="flex text-surface2 gap-x-2 absolute top-[-18%] left-[-8%]"
            >
                <div class="flex flex-col justify-end w-fit gap-y-3">
                    <Plus />
                    <Plus />
                </div>
                <div class="flex flex-col gap-y-3">
                    <Plus />
                    <Plus />
                    <Plus />
                    <Plus />
                </div>
                <div class="flex flex-col gap-y-3">
                    <Plus />
                    <Plus />
                </div>
            </div>
            <div
                class="flex text-surface2 gap-x-2 absolute top-[86.5%] left-[97.7%]"
            >
                <div class="flex flex-col justify-end w-fit gap-y-3">
                    <Plus />
                    <Plus />
                </div>
                <div class="flex flex-col gap-y-3">
                    <Plus />
                    <Plus />
                    <Plus />
                    <Plus />
                </div>
                <div class="flex flex-col gap-y-3">
                    <Plus />
                    <Plus />
                </div>
            </div>
        </div>
    </div>
</template>

