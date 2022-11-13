<template>
<div class="w-screen overflow-y-scroll no-scrollbar h-screen bg-base font-semibold text-text px-44 pt-44">
    <div class="text-4xl font-bold">Deploy a new plugin</div>
    <div class="text-overlay1 mt-3">Thanks for taking time to create a plugin! Just select your GitHub repo that contains your source code for the plugin and the rest will be handled by takoyaki (building for production)</div>
    <div class="repos mt-8 flex gap-x-8 flex-wrap gap-y-8 w-fit">
        <a class="repo bg-surface0/20 rounded-lg w-[450px] p-6 px-8 h-auto flex flex-col" v-for="repo in repos" :key="repo.id" :href="'/deploy/' + repo.full_name">
            <div class="state flex items-start gap-x-2 text-overlay1 font-semibold">
                <vue-feather type="lock" size="14" /><div class="text-sm">{{ repo.private ? "Private" : "Public" }}</div>
            </div>
            <div class="name text-xl font-bold mt-2 mb-1">{{ repo.name }}</div>
            <div class="desc font-semibold text-overlay1 mb-4">{{ repo.description }}</div>
            <div class="stats mt-auto flex gap-x-4 text-overlay2">
                <div class="stat flex items-center text-green">
                    <vue-feather type="star" stroke-width="3" size="16" class="mr-2" />
                    <div class="text-sm pt-0.5">{{ repo.stargazers_count }}</div>
                </div>
                <div class="stat flex items-center text-blue">
                    <vue-feather type="git-branch" stroke-width="3" size="16" class="mr-2" />
                    <div class="text-sm pt-0.5">{{ repo.forks_count }}</div>
                </div>
                <div class="stat flex items-center text-red">
                    <vue-feather type="disc" stroke-width="3" size="16" class="mr-2" />
                    <div class="text-sm pt-0.5">{{ repo.open_issues_count }}</div>
                </div>
            </div>
        </a>
    </div>
</div>
</template>

<script lang="ts" setup>
import { getAuth , onAuthStateChanged } from 'firebase/auth';
import {ref} from 'vue';
import Lock from "../components/Lock.vue"

const repos = ref<Array<any>>([]);
const token = localStorage.getItem("GITHUB_ACCESS_TOKEN")

onAuthStateChanged(getAuth() , async (user) => {
    if(user) {
        repos.value = (await (await fetch(`https://api.github.com/search/repositories?q=user:${user.displayName}` , {
            headers: {
                "Authorization": `Bearer ${token}`
            }
        })).json()).items
    }
})
</script>
