import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { Client } from "appwrite";
import { plugin , createMetaManager } from "vue-meta";
import VueFeather from "vue-feather";

import "./assets/main.sass";

const appwrite = new Client();

appwrite
    .setProject("63567282258749c8b86e")
    .setEndpoint("http://appwrite.kyeboard.me/v1");

const app = createApp(App)
    .use(router)
    .use(createMetaManager())
    .component(VueFeather.name, VueFeather)
    .provide("appwrite", appwrite)
    .mount("#app");

