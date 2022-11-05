import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { initializeApp } from "firebase/app";

import "./assets/main.sass";

const app = createApp(App);

app.use(router);

const firebaseConfig = {
    apiKey: "AIzaSyC1o08v2I0zytj-2Bq638Di4CHpfo_8ocg",
    authDomain: "kyeboard-takoyaki.firebaseapp.com",
    projectId: "kyeboard-takoyaki",
    storageBucket: "kyeboard-takoyaki.appspot.com",
    messagingSenderId: "974109853641",
    appId: "1:974109853641:web:3cfa4f7ed15d4bd2f08a18",
    measurementId: "G-9H7G7WM4RG",
};

// Initialize Firebase
app.provide("firebase", initializeApp(firebaseConfig));

app.mount("#app");
