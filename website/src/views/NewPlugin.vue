<template>
    <div class="w-screen h-screen bg-base pt-44 px-44 text-text">
        <div class="text-3xl font-bold w-full mb-2">
            Publish a new plugin
        </div>
        <div class="w-full mb-8 text-overlay1 font-semibold">
            Thanks for taking the time to build a plugin for takoyaki!
        </div>
        <div class="data text-text">
            <div class="font-bold">Name</div>
            <input
                type="text"
                class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                placeholder="A great name for your plugin...."
            />
            <div class="font-bold mt-6">Description</div>
            <input
                type="text"
                class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                placeholder="A short description about your plugin..."
            />
            <div class="font-bold mt-6">Logo</div>
            <input
                type="text"
                class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                placeholder="Nothing is better than a logo! Paste in the URL of your logo"
            />
            <div class="font-bold mt-6">Binary</div>
            <input
                type="text"
                class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                placeholder="Your plugin's build file, also known as the binary file"
            />
        </div>
        <button class="w-full p-4 rounded-xl mt-8 bg-gradient-to-r from-[#eecda3] to-[#ef629f] text-base font-semibold mt-4">
            Publish
        </button>
    </div>
</template>
