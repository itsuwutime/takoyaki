<template>
<div class="w-screen overflow-y-scroll h-screen bg-base font-semibold text-text px-44 pt-44">
    <div class="text-4xl font-bold">Deploy a new plugin</div>
    <div class="text-overlay1 mt-3">Thanks for taking time to create a plugin! Just select your GitHub repo that contains your source code for the plugin and the rest will be handled by takoyaki (building for production)</div>
    {{repos}}
</div>
</template>

<script lang="ts" setup>
import { getAuth , onAuthStateChanged } from 'firebase/auth';
import {ref} from 'vue';

const repos = ref<any>(null);
const token = localStorage.getItem("GITHUB_ACCESS_TOKEN")

onAuthStateChanged(getAuth() , async (user) => {
    if(user) {
        repos.value = await (await fetch(`https://api.github.com/search/repositories?q=user:${user.displayName}` , {
            headers: {
                "Authorization": `Bearer ${token}`
            }
        })).json()
    }
})
</script>
