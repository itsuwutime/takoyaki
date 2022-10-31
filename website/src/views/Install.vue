<template>
    <div class="w-screen h-screen bg-base pt-44 px-44">
        <div class="text-3xl font-bold text-text">
            Select your operating system
        </div>
        <div class="text-overlay1 font-semibold mt-2">
            Select your daily driver
        </div>
        <div class="flex text-text mt-4 gap-x-4">
            <div
                class="os bg-surface0/20 p-3 w-56 px-4 pb-5 rounded-xl arch flex flex-col items-center justify-center font-bold"
                @click="select('arch')"
            >
                <i class="fl-archlinux text-4xl m-2 mb-2 text-blue" /> Arch Linux 
            </div>
            <div
                class="os bg-surface0/20 p-3 w-56 px-4 pb-5 rounded-xl ubuntu flex flex-col items-center justify-center font-bold"
                @click="select('ubuntu')"
            >
                <i class="fl-ubuntu text-4xl mr-3 m-2 text-peach" /> Ubuntu
            </div>
            <div
                class="os bg-surface0/20 p-3 w-56 px-4 pb-5 rounded-xl macos flex flex-col items-center justify-center font-bold"
                @click="select('macos')"
            >
                <i class="fl-apple text-3xl mr-3 m-2 text-text" /> MacOS
            </div>
            <div
                class="os bg-surface0/20 p-3 w-56 px-4 pb-5 rounded-xl termux flex flex-col items-center justify-center font-bold"
                @click="select('termux')"
            >
                <i class="fa-brands fa-android text-4xl mr-3 mb-1 mx-2 text-green" /> Termux
            </div>
            <div
                class="os bg-surface0/40 p-3 w-56 px-4 pb-5 rounded-xl flex windows flex-col items-center justify-center font-bold"
                @click="select('windows')"
            >
                <i class="fa-brands fa-windows text-4xl mr-3 m-2 text-sky" /> Windows
            </div>
        </div>
        <div class="guide mt-10 text-text">
            <div class="text-2xl font-bold">Installation guide</div>
            <div class="text-overlay1 mt-2 font-semibold">
                Make sure to copy all the commands carefully!
            </div>

            <div class="mt-6 font-bold text-text">
                Open up your terminal and run the following commands
            </div>
            <div
                class="code p-4 px-6 rounded-xl bg-surface0/20 mt-4 text-text tracking-wide font-semibold"
            >
                $ {{ command }}
            </div>

            <div class="mt-6 font-bold text-text">
                Once installed, you can now start using takoyaki!
            </div>
            <div
                class="code p-4 px-6 rounded-xl bg-surface0/20 mt-4 text-text tracking-wide font-semibold"
            >
                $ takoyaki --help
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { useMeta } from "vue-meta";

const current_os = ref<string>("");
const command = ref<string>("yay -S takoyaki");

const select = (name: string) => {
    current_os.value = name;
};

useMeta({
    title: "Install takoyaki - blazingly fast!"
})

watch(current_os, (new_val) => {
    document.querySelectorAll(".os").forEach((x) => {
        x.classList.remove(...["bg-surface0/40"]);
        x.classList.add(...["bg-surface0/20"]);
    });

    document
        .querySelector("." + new_val)
        ?.classList.remove(...["bg-surface0/20"]);
    document
        .querySelector("." + new_val)
        ?.classList.add(...["bg-surface0/40"]);

    switch (new_val) {
        case "arch":
            command.value = "yay -S takoyaki";
            break;
        case "ubuntu":
            command.value = "curl https://usetakoyaki.kyeboard.xyz | sh";
            break;
        case "macos":
            command.value = "curl https://usetakoyaki.kyeboard.xyz | sh";
            break;
        case "termux":
            command.value = "curl https://usetakoyaki.kyeboard.xyz | sh";
            break;
        case "windows":
            command.value = "curl https://usetakoyaki.kyeboard.xyz | sh";
            break;
    }
});

// Initial
select("arch");
</script>
