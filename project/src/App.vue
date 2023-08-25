<script setup>
import buttonImportant from './components/molecules/buttonImportant.vue'
import { ref } from 'vue'

const figures = ref([
    {
        name: 'cone',
        img: 'circle.png'
    },
    {
        name: 'truncated cone',
        img: 'circle.png'
    },
    {
        name: 'semiesphere',
        img: 'circle.png'
    }
])
</script>

<template>
    <header>Phet a la Tortrix</header>

    <div class="figures-bar">
        <buttonImportant
            class="figures-btn"
            v-for="figure in figures"
            :key="figure.name">
            <div class="figures-btn-content">
                <img :src="figure.img" alt="circle" />
                <span>{{ figure.name }}</span>
            </div>
        </buttonImportant>
    </div>
</template>

<style scoped>
header {
    font-size: 2rem;
    width: 100%;
    text-align: center;
    padding: 1ch;
}

.figures-bar {
    width: fit-content;
    margin: auto;
    display: flex;
    align-items: center;
}

.figures-btn {
    margin: 0 2ch;
}

.figures-btn-content {
    width: 8rem;
    margin: 0 2ch;
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
    font-size: 1rem;
}
</style>
