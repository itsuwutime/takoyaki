<template>
    <div class="w-screen h-screen bg-base px-44 pt-44 text-text flex">
        <div
            class="flex w-fit pr-8 border-r border-r-surface0 flex-col justify-center gap-y-6 h-full font-semibold text-overlay1"
        >
            <div
                class="content hover:text-text transition-all cursor-pointer"
                v-for="doc in docs.docs"
                :key="doc.name"
            >
                {{ doc.name }}<br />
            </div>
        </div>
        <div class="w-full pb-10 content ml-8 overflow-scroll h-full no-scrollbar" v-html="html_doc" />
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { marked } from "marked";
import DOMPurify from "dompurify";
import { useRoute } from "vue-router";
import hljs from "highlight.js";
import "highlight.js/styles/nord.css"

marked.setOptions({
    highlight: function(code, lang) {
        return hljs.highlight(lang, code).value;
    }
});

interface Root {
    docs: Array<Docs>;
}

interface Docs {
    name: string;
    file: string;
}

const docs = ref<Root>({ docs: [] });
const route = useRoute();
const html_doc = ref<string>("");

(async () => {
    docs.value = await (await fetch("/docs/docs.json")).json();

    html_doc.value = DOMPurify.sanitize(
        marked.parse(
            await (await fetch("/docs/" + route.params.id + ".md")).text()
        )
    );
})();
</script>
