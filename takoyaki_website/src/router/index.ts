import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import MarketPlace from "../views/Marketplace.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "Home",
            component: HomeView,
        },
        {
            path: "/marketplace",
            name: "Marketplace",
            component: MarketPlace,
        },
    ],
});

export default router;
