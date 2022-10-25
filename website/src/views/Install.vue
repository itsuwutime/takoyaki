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
                class="os bg-surface0/40 p-3 w-56 px-4 rounded-xl arch flex items-center justify-center font-bold"
                @click="select('arch')"
            >
                <i class="fl-archlinux text-2xl mr-2" /> Arch Linux
            </div>
            <div
                class="os bg-surface0/40 p-3 w-56 px-4 rounded-xl ubuntu flex items-center justify-center font-bold"
                @click="select('ubuntu')"
            >
                <i class="fl-ubuntu text-2xl mr-3" /> Ubuntu
            </div>
            <div
                class="os bg-surface0/40 p-3 w-56 px-4 rounded-xl macos flex items-center justify-center font-bold"
                @click="select('macos')"
            >
                <i class="fl-apple text-2xl mr-3" /> MacOS
            </div>
            <div
                class="os bg-surface0/40 p-3 w-56 px-4 rounded-xl termux flex items-center justify-center font-bold"
                @click="select('termux')"
            >
                <i class="fa-brands fa-android text-2xl mr-3" /> Termux
            </div>
            <div
                class="os bg-surface0/40 p-3 w-56 px-4 rounded-xl flex windows items-center justify-center font-bold"
                @click="select('windows')"
            >
                <i class="fa-brands fa-windows text-2xl mr-3" /> Windows
            </div>
        </div>
        <div class="guide mt-10 text-text">
            <div class="text-2xl font-bold">Installation guide</div>
            <div class="text-overlay1 mt-2 font-semibold">
                Make sure to follow all the instructions properly
            </div>

            <div class="mt-6 font-bold text-text">
                Open up your terminal and run the following commands
            </div>
            <div
                class="code p-4 px-6 rounded-xl bg-surface0/40 mt-2 text-overlay2 tracking-wide font-semibold"
            >
                <!-- $ curl https://usetakoyaki.kyeboard.xyz | sh -->
                $ {{ command }}
            </div>

            <div class="mt-6 font-bold text-text">
                Once installed, you can now start using takoyaki!
            </div>
            <div
                class="code p-4 px-6 rounded-xl bg-surface0/40 mt-2 text-overlay2 tracking-wide font-semibold"
            >
                $ takoyaki --help
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

const current_os = ref<string>("");
const command = ref<string>("yay -S takoyaki");

const select = (name: string) => {
    current_os.value = name;
};

watch(current_os, (new_val) => {
    document.querySelectorAll(".os").forEach((x) => {
        x.classList.remove(...["bg-blue/100", "text-base"]);
        x.classList.add(...["bg-surface0/40", "text-text"]);
    });

    document
        .querySelector("." + new_val)
        ?.classList.remove(...["bg-surface0/40", "text-text"]);
    document
        .querySelector("." + new_val)
        ?.classList.add(...["bg-blue/100", "text-base"]);

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
