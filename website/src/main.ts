import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { Client } from "appwrite";
import VueFeather from "vue-feather";

import "./assets/main.sass";

const appwrite = new Client();

appwrite
    .setProject("63567282258749c8b86e")
    .setEndpoint("http://appwrite.kyeboard.me/v1");

const app = createApp(App);

app.use(router);

app.component(VueFeather.name, VueFeather);

app.provide("appwrite", appwrite);

app.mount("#app");
