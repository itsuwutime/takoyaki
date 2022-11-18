import { createApp } from "vue";
import VueFeather from 'vue-feather';
import App from "./App.vue";
import router from "./router";
import { createMetaManager } from 'vue-meta'
import { initializeApp } from "firebase/app";

import "./assets/main.sass";

const app = createApp(App);

app.use(router);

const firebaseConfig = {
    apiKey: "AIzaSyDV8RkplTsJa9NXueaUUunH7_OjxfIydEc",
    authDomain: "kyeboard-takoyaki-ae3b1.firebaseapp.com",
    databaseURL: "https://kyeboard-takoyaki-ae3b1-default-rtdb.asia-southeast1.firebasedatabase.app",
    projectId: "kyeboard-takoyaki-ae3b1",
    storageBucket: "kyeboard-takoyaki-ae3b1.appspot.com",
    messagingSenderId: "738243958501",
    appId: "1:738243958501:web:9847e04f15b6d60a61660c",
    measurementId: "G-PSB588WN8M"
};

// Initialize Firebase
app.provide("firebase", initializeApp(firebaseConfig));

// Vue components
app.component(VueFeather.name, VueFeather);
app.use(createMetaManager())

// Mount
app.mount("#app");
