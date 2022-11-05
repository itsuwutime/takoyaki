<template>
    <div
        class="navbar px-16 w-full fixed items-center justify-center text-text font-semibold top-0 left-0 py-6 flex border-b border-b-surface0/60"
    >
        <div class="left flex items-center justify-center">
            <img src="/logo.png" class="h-10" />
            <div class="ml-6 text-text font-bold text-lg">takoyaki</div>
        </div>
        <div class="mx-auto text-overlay2 links flex gap-x-14">
            <a href="/">Home</a>
            <a href="/installation">Installation</a>
            <a href="/documentation">Documentation</a>
            <a href="/marketplace">Marketplace</a>
        </div>
        <div
            class="current-user flex items-center justify-center"
            v-if="current_user != null"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.4"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="feather feather-log-out"
            >
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                <polyline points="16 17 21 12 16 7"></polyline>
                <line x1="21" y1="12" x2="9" y2="12"></line>
            </svg>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                class="ml-10"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="#cdd6f4"
                stroke-width="2.4"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
            </svg>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                viewBox="0 0 24 24"
                class="ml-10"
                fill="none"
                stroke="#cdd6f4"
                stroke-width="2.4"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="11" cy="11" r="8"></circle>
                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
            </svg>
            <img
                :src="
                    current_user.photoURL ||
                    'https://www.gravatar.com/avatar/00000000000000000000000000000000'
                "
                alt="User profile"
                class="h-10 rounded-full ml-16"
            />
        </div>
        <button
            class="login bg-surface0/40 text-text p-3 px-10 rounded-lg"
            @click="oauth_with_github()"
            v-else
        >
            Login
        </button>
    </div>
</template>

<script setup lang="ts">
import {
    GithubAuthProvider,
    getAuth,
    signInWithPopup,
    onAuthStateChanged,
} from "firebase/auth";
import type { User } from "firebase/auth";
import { ref } from "vue";
import { useRouter } from "vue-router";

const provider = new GithubAuthProvider();
const auth = getAuth();
const current_user = ref<User | null>(null);
const router = useRouter();

provider.addScope("repo");

onAuthStateChanged(auth, (user) => {
    if (user) {
        current_user.value = user;
    }

    console.log(user);
});

const oauth_with_github = async () => {
    const res = await signInWithPopup(auth, provider);

    router.push({ path: "/user/" + res.user.displayName });
};
</script>
