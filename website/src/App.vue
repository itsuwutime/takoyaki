<script setup lang="ts">
import { RouterView } from "vue-router";
import NavBar from "./components/NavBar.vue";
</script>

<template>
    <metainfo>
        <template v-slot:title="{ content }">{{ content }}</template>
    </metainfo>
    <NavBar />
    <RouterView />
</template>
