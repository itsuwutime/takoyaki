<script setup lang="ts">
import NavBar from "./components/NavBar.vue";
</script>

<template>
    <metainfo>
        <template v-slot:title="{ content }">{{ content ? `${content} | takoyaki` : `takoyaki` }}</template>
    </metainfo>
    <NavBar />
    <RouterView />
</template>

