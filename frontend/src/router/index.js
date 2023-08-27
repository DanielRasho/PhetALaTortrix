import { createRouter, createWebHistory } from 'vue-router'
import Empty from '../views/Empty.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path:"/",
            redirect: {path: "/PhetALaTortrix"}
        },
        {
            path: '/PhetALaTortrix',
            component: Empty,
            children: [
                {
                    path: 'cone',
                    name: 'Cone',
                    component: () => import('../views/ConeTrunk.vue')
                },
                {
                    path: 'trunk',
                    name: 'ConeTrunk',
                    component: () => import('../views/ConeTrunk.vue')
                },
                {
                    path: 'hemispher',
                    name: 'Hemisphere',
                    component: () => import('../views/Hemisphere.vue')
                }
            ]
        }
    ]
})

export default router
