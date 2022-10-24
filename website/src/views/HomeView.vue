<script setup lang="ts">
import {inject} from 'vue';
import { Account , Client } from "appwrite"

const appwrite = inject("appwrite") as Client;
const account = new Account(appwrite);

account.createOAuth2Session("google")
</script>

<template>
<div class="w-screen h-screen bg-black"></div>
</template>
