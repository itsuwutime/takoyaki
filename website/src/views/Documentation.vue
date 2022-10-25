<template>
<div class="w-screen h-screen bg-base px-44 pt-44 text-text">
    <div class="flex flex-col gap-y-6 w-full h-full font-semibold text-overlay1">
        <div class="content hover:text-text transition-all cursor-pointer" v-for="doc in docs.docs" :key="doc.name">{{doc.name}}<br></div>
    </div>
</div>
</template>

<script setup lang="ts">
import {ref} from 'vue';

interface Root {
    docs: Array<Docs>
}

interface Docs {
    name: string
    file: string
}

const docs = ref<Root>({ docs: [] });

(async () => {
    docs.value = await (await fetch("/docs/docs.json")).json()
})()
</script>
