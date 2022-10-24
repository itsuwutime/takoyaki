<script setup lang="ts">
import SideBar from "../components/SideBar.vue"
</script>

<template>
<SideBar />
<div class="w-screen h-screen bg-base flex-col-reverse md:flex-row flex items-center justify-center text-text md:pl-44 md:pr-10 px-[5%]">
    <div class="info md:w-1/2 mr-20 mt-10 md:mt-0">
        <div class="text-5xl font-bold leading-[60px]">Blazingly fast git contribution graph in your terminal.</div>
        <div class="text-overlay1 font-semibold mt-4">Takoyaki is a blazingly fast CLI tool that prints your git contributions in your terminal elegantly. It is highly customizable with support of bunch of popular cloud based git repository providers.</div>
        <div class="features mt-8 flex flex-col gap-y-4 font-semibold">
            <div class="flex items-center">  
                <vue-feather type="check-circle" size="19" class="mr-3 text-blue" /> Blazingly fast
            </div>
            <div class="flex items-center">  
                <vue-feather type="check-circle" size="19" class="mr-3 text-blue" /> Highly customizable
            </div>
            <div class="flex items-center">  
                <vue-feather type="check-circle" size="19" class="mr-3 text-blue" /> Open source
            </div>
            <div class="flex items-center">  
                <vue-feather type="check-circle" size="19" class="mr-3 text-blue" /> Plugins that support popular providers
            </div>
        </div>
    </div>
    <img src="/banner.png" class="md:w-1/2 ml-auto" />
</div>
</template>
