<template>
    <div class="w-screen h-screen px-44 pt-44 bg-base">
        <div class="top_bar flex">
            <div class="search bg-surface0/20 p-4 w-full rounded-lg">
                <div class="prefix flex px-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="18"
                        height="18"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke-width="2.4"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="feather stroke-text feather-search"
                    >
                        <circle cx="11" cy="11" r="8"></circle>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                    </svg>
                    <input class="w-full h-full bg-transparent outline-none text-text ml-6 placeholder-overlay1" placeholder="Search for plugins..." />
                </div>
            </div>
            <a href="/new">
                <button class="bg-blue w-52 ml-6 text-base rounded-lg font-semibold h-full">New plugin</button>
            </a>
        </div>
    </div>
</template>
