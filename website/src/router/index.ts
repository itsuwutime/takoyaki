import { createRouter, createWebHistory } from "vue-router";
import Marketplace from "../views/Marketplace.vue"
import HomeView from "../views/HomeView.vue";
import Install from "../views/Install.vue";
import Me from "../views/Me.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: HomeView,
        },
        {
            path: "/install",
            component: Install,
        },
        {
            path: "/me",
            component: Me,
        },
        {
            path: "/marketplace",
            component: Marketplace,
        },
    ],
});

export default router;
