<template>
    <div class="w-screen h-screen bg-base pt-32 px-44 text-text">
        <div class="text-3xl font-bold text-center w-full mb-6">
            Publish a new plugin
        </div>
        <div class="flex">
            <div class="flex flex-col justify-center w-1/2 text-text">
                <div class="mb-2">Display picture</div>
                <label
                    for="dropzone-file"
                    class="flex flex-col justify-center items-center w-full h-full bg-surface0/40 rounded-lg cursor-pointer"
                >
                    <div
                        class="flex flex-col justify-center items-center pt-5 pb-6"
                    >
                        <p class="mb-2 text-sm">
                            <span class="font-semibold">Click to upload</span>
                            or drag and drop
                        </p>
                        <p class="text-xs">
                            PNG or JPG (300x300 for the best display)
                        </p>
                    </div>
                    <input id="dropzone-file" type="file" class="hidden" />
                </label>
            </div>
            <div class="data w-1/2 text-text ml-10">
                <div>Name</div>
                <input
                    type="text"
                    class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                    placeholder="A great name for your plugin...."
                />
                <div class="mt-6">Description</div>
                <input
                    type="text"
                    class="w-full p-4 rounded-xl px-6 bg-surface0/40 focus:outline-none focus:border-none mt-1 placeholder-overlay1"
                    placeholder="A short description about your plugin..."
                />
            </div>
        </div>
        <br /><br />
        <div class="flex flex-col justify-center w-full h-52 text-text">
            <div class="mb-2">Binary file for your plugin</div>
            <label
                for="dropzone-file"
                class="flex flex-col justify-center items-center w-full h-full bg-surface0/40 rounded-lg cursor-pointer"
            >
                <div
                    class="flex flex-col justify-center items-center pt-5 pb-6"
                >
                    <p class="mb-2 text-sm">
                        <span class="font-semibold">Click to upload</span> or
                        drag and drop
                    </p>
                    <p class="text-xs">Binary file must be less than 100MB</p>
                </div>
                <input id="dropzone-file" type="file" class="hidden" />
            </label>
        </div>
        <button class="w-full p-4 rounded-xl bg-blue text-base font-bold mt-4">
            Publish
        </button>
    </div>
</template>
