import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        { path: "/", component: import('../views/Cone.vue')},
        { path: "/trunk", component: () => import('../views/ConeTrunk.vue')},
        { path: "/hemisphere", component: () => import('../views/Hemisphere.vue')},
    ]
})

export default router
