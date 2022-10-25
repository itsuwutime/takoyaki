<template>
    <div class="w-screen h-screen bg-base"></div>
</template>

<script lang="ts" setup>
import { Account, type Client } from "appwrite";
import { inject } from "vue";

const appwrite = inject("appwrite") as Client;
const account = new Account(appwrite);

(async () => {
    try {
        await account.get();
    } catch (err) {
        account.createOAuth2Session("github", "http://localhost:3000/me");
    }
})();
</script>
