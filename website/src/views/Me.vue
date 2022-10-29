<template>
    <div class="w-screen text-text h-screen bg-base flex items-center flex-col justify-center">
        <img src="/loading.gif" />
        <div class="text-2xl font-bold mt-4">Checking for existing login...</div>
    </div>
</template>

<script lang="ts" setup>
import { Account, type Client } from "appwrite";
import { inject } from "vue";

const appwrite = inject("appwrite") as Client;
const account = new Account(appwrite);

// (async () => {
//     try {
//         await account.get();
//     } catch (err) {
//         account.createOAuth2Session("github", "http://localhost:3000/me");
//     }
// })();
</script>
