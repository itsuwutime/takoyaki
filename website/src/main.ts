import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { Client } from "appwrite"

import "./assets/main.sass";

const appwrite = new Client();

appwrite
    .setProject("6355ff83c65adf66a6b1")
    .setEndpoint("https://okasa.centralindia.cloudapp.azure.com/v1")

const app = createApp(App);

app.use(router);

app.provide("appwrite", appwrite)

app.mount("#app");
