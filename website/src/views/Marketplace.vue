<template>
    <div class="w-screen h-screen bg-base text-text pt-44 px-44">
        <div class="text-3xl font-bold">Plugins</div>
        <div class="text-overlay1 font-semibold mt-2">Awesome plugins built by awesome developers</div>
        <div class="mt-10 flex gap-x-12 flex-wrap gap-y-6">
            <a class="plugin bg-surface0/20 p-8 rounded-lg w-[500px]" v-for="plugin in plugins.plugins" :key="plugin.name" :href="'/plugin/' + plugin.name.toLowerCase()">
                <div class="flex flex-col h-full">
                    <img :src="plugin.image" class="w-[70px] mr-4 float-left" />
                    <div class="info mt-5 mb-4">
                        <div class="title text-xl font-bold">{{plugin.name}}</div>
                        <div class="title text-overlay2 font-semibold mt-2 leading-7">{{plugin.description}}</div>
                    </div>
                    <div class="mt-auto flex items-center font-semibold text-blue">
                        <vue-feather type="link" size="19" class="mr-2" /> <a :href="plugin.home_page">{{plugin.home_page}}</a>
                    </div>
                </div>
            </a>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useMeta } from "vue-meta";

useMeta({
    title: "Plugins - Marketplace - takoyaki"
})

interface PluginType {
    name: string,
    home_page: string,
    description: string,
    image: string
}

interface Plugins {
    plugins: Array<PluginType> 
}

const plugins = ref<Plugins>({ plugins: [] });

(async () => {
    const res = await fetch("https://raw.githubusercontent.com/kyeboard/takoyaki/main/plugins/plugins.json");

    plugins.value = await res.json();
})()
</script>

