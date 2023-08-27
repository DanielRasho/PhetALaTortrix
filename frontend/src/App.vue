<script setup>
import buttonImportant from '@/components/atoms/buttonImportant.vue'
import { useRouter, RouterView } from 'vue-router'

import { ref } from 'vue'

const router = useRouter()
const figures = ref([
    {
        name: 'Cone',
        img: 'circle.png',
        view: 'Cone'
    },
    {
        name: 'Truncated Cone',
        img: 'circle.png',
        view: 'ConeTrunk'
    },
    {
        name: 'Hemisphere',
        img: 'circle.png',
        view: 'Hemisphere'
    }
])

function changeFormView(viewPath) {
    router.replace({ name: viewPath })
}
</script>

<template>
    <div class="app">
        <nav class="figures-bar">
            <buttonImportant
                class="figures-btn"
                v-for="figure in figures"
                :key="figure.name"
                @click.prevent="changeFormView(figure.view)"
            >
                <div class="figures-btn-content">
                    <img :src="figure.img" alt="circle" />
                    <span>{{ figure.name }}</span>
                </div>
            </buttonImportant>
        </nav>
        <router-view v-slot="{ Component }">
            <transition name="fade" mode="out-in">
                <component :is="Component" />
            </transition>
        </router-view>
    </div>
</template>

<style scoped>
.app {
    display: flex;
}

.figures-bar {
    width: 250px;
    height: 100vh;
    flex-shrink: 0;

    padding: 0 3ch;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.figures-btn {
    margin: 2ch 0;
}

.figures-btn-content {
    max-width: 9rem;
    margin: 0 1ch;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-radius: 1rem;
    background-color: var(--primary-button);
}
.figures-btn-content img {
    width: 70%;
}

.figures-btn-content span {
    font-size: 1.2rem;
    color: var(--background);
    font-weight: 700;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease-out;
}
</style>
