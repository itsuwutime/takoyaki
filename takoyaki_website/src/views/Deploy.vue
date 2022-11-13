<template>
<div class="bg-base w-screen h-screen text-text px-44 pt-44">
    <div class="text-4xl font-bold">You're almost done!</div>
    <div class="text-overlay1 mt-3">Just configure some stuff for your application and get your plugin up and ready to be used by others!</div>
    <div class="mt-16 flex">
        <div class="git_repo_info w-96">
            <div class="uppercase font-bold text-sm text-overlay1">GIT REPOSITORY</div>
            <div class="github mt-4 font-bold flex items-center text-overlay3">
                <vue-feather type="github" size="20" class="text-overlay1 mr-4" />
                <div class="pb-1">{{username}}/{{repository}}</div>
            </div>
            <div class="github mt-2 font-bold flex items-center text-overlay3">
                <vue-feather type="git-branch" size="20" class="text-overlay1 mr-4" />
                <div class="pb-1">{{branch}}</div>
            </div>
            <div class="github mt-2 font-bold flex items-center text-overlay3">
                <vue-feather type="folder" size="20" class="text-overlay1 mr-4" />
                <div class="pb-1">{{folder}}</div>
            </div>
            <br />
            <hr class="border-surface0/40 border w-full" />
            <br />
            <a href="/new" class="flex items-center font-semibold tracking-wide text-overlay1">
                Import a different Git Repository
                <vue-feather type="arrow-right" size="20" class="ml-2" />
            </a>
            <a href="/marketplace" class="mt-4 flex items-center font-semibold tracking-wide text-overlay1">
                Browse plugins
                <vue-feather type="arrow-right" size="20" class="ml-2" />
            </a>
        </div>
        <div class="w-full ml-24">
            <div class="text-3xl font-bold">Configure plugin</div>
            <br>
            <hr class="border-surface0/40 border-2 rounded-lg">
            <br>
            <div class="input-group font-semibold">
                <div class="text-overlay1 text-sm">Name</div>
                <input class="p-3 rounded-lg bg-surface0/20 w-full mt-2 px-5 placeholder-overlay1 text-overlay2 focus:outline-none" placeholder="my-awesome-plugin" />
            </div>
            <br>
            <div class="input-group font-semibold">
                <div class="text-overlay1 text-sm">Path</div>
                <input class="p-3 rounded-lg bg-surface0/20 w-full mt-2 px-5 placeholder-overlay1 text-overlay2 focus:outline-none" placeholder="/" />
            </div>
            <br>
            <div class="input-group font-semibold">
                <div class="text-overlay1 text-sm">Branch</div>
                <input class="p-3 rounded-lg bg-surface0/20 w-full mt-2 px-5 placeholder-overlay1 text-overlay2 focus:outline-none" placeholder="main" />
            </div>
            <br>
            <button class="w-full bg-blue text-base p-3 rounded-lg font-semibold">Deploy</button>
        </div>
    </div>
</div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const username = route.params.username;
const repository = route.params.repository;
const branch = ref<String>("main");
const folder = ref<String>("/")
</script>
