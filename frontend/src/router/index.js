import { createRouter, createWebHistory } from 'vue-router'
import Cone from '../views/Cone.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        { path: '/', name: 'Cone', component: Cone },
        {
            path: '/trunk',
            name: 'ConeTrunk',
            component: () => import('../views/ConeTrunk.vue')
        },
        {
            path: '/hemisphere',
            name: 'Hemisphere',
            component: () => import('../views/Hemisphere.vue')
        }
    ]
})

export default router
