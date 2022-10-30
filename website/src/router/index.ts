import { createRouter, createWebHistory } from "vue-router";
import Marketplace from "../views/Marketplace.vue";
import HomeView from "../views/HomeView.vue";
import Install from "../views/Install.vue";
import NewPlugin from "../views/NewPlugin.vue";
import Documentation from "../views/Documentation.vue"
import Plugin from "../views/Plugin.vue"
import DocumentationEmpty from "../views/Documentation_Empty.vue"

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            component: HomeView,
        },
        {
            path: "/install",
            component: Install,
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
        {
            path: "/documentation",
            component: DocumentationEmpty,
        },
        {
            path: "/plugin/:id",
            component: Plugin,
        },
    ],
});

export default router;
