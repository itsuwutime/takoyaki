<template>
<div class="w-screen h-screen bg-base px-44 text-text pt-44">
    <div class="plugin" v-if="_plugin">
        <img :src="_plugin.image" class="w-44" />
        <div class="info mt-8">
            <div class="title text-2xl font-bold">{{_plugin.name}}</div>
            <div class="desc text-overlay1 font-semibold mt-2 tracking-wide">{{_plugin.description}}</div>
        </div>
        <a :href="_plugin.home_page" class="url flex items-center font-semibold mt-6 text-blue">
            <vue-feather type="link" class="mr-4" /> {{_plugin.home_page}}
        </a>
        <a :href="_plugin.developer_url" class="url flex items-center font-semibold mt-6 text-blue">
            <vue-feather type="github" class="mr-4" /> {{_plugin.developer_url}}
        </a>
        <br>
        <hr class="border-surface0">
        <br>
        <div class="install">
            <div class="title font-semibold">To install this plugin, run the following command in your terminal</div>
            <div class="bg-surface0/20 p-4 rounded-lg mt-2 px-6">$ takoyaki install {{_plugin.name.toLowerCase()}}</div>
        </div>
        <div class="use mt-6">
            <div class="title font-semibold">To use this plugin, run the following command in your terminal</div>
            <div class="bg-surface0/20 p-4 rounded-lg mt-2 px-6">$ takoyaki use {{_plugin.name.toLowerCase()}}</div>
        </div>
    </div>
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import {useRoute, useRouter} from 'vue-router';
import { useMeta } from 'vue-meta';

const _plugin = ref<any>(null);
const route = useRoute();
const router = useRouter();

(async () => {
    const plugins = await (await fetch("https://raw.githubusercontent.com/kyeboard/takoyaki/main/plugins/plugins.json")).json();

    for(const plugin of plugins.plugins) {
        if(route.params.id == plugin.name.toLowerCase()) {
            _plugin.value = plugin;

            break;
        }
    }

    if(!_plugin.value) {
        return router.push({ path: "/marketplace" })
    }
})()

useMeta({
    title: `Install plugin - takoyaki`
})

</script>
