import Vue from "vue"
import VueRouter from "vue-router"
import Home from "../views/Home.vue"
import ErrorNotFound from "../components/ErrorNotFound"

Vue.use(VueRouter);

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home
    },

    {
        path: "/project/:pid",
        name: "Project",
        component: () => import(/* webpackChunkName: "environment" */ "../views/Environment.vue")
    },

    {
        path: "/about",
        name: "About",
        component: () => import(/* webpackChunkName: "about" */ "../views/About.vue")
    },

    {
        path: "*",
        name: "404",
        component: ErrorNotFound,
    }
];

const router = new VueRouter({
    routes,
});

export default router
