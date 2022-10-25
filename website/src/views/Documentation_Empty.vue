<template>
<div class="w-screen h-screen bg-base"></div>
</template>

<script lang="ts" setup>
import {useRouter} from 'vue-router';

const router = useRouter();

router.push({ path: "/documentation/getting-started" })
</script>

