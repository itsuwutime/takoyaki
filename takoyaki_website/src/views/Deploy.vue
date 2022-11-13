<template>
<div class="bg-base w-screen h-screen text-text px-44 pt-44">
    <div class="text-4xl font-bold">You're almost done!</div>
    <div class="text-overlay1 mt-3">Just configure some stuff for your application and get your plugin up and ready to be used by others!</div>
    <div class="mt-10">
        <div class="git_repo_info">
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
