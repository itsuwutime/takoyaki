import { createRouter, createWebHistory } from "vue-router";
import Marketplace from "../views/Marketplace.vue";
import HomeView from "../views/HomeView.vue";
import Install from "../views/Install.vue";
import NewPlugin from "../views/NewPlugin.vue";
import Me from "../views/Me.vue";
import Documentation from "../views/Documentation.vue";

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
        {
            path: "/marketplace/new",
            component: NewPlugin,
        },
        {
            path: "/documentation/:id",
            component: Documentation,
        },
    ],
});

export default router;
