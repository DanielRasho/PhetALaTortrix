<script setup>
import buttonImportant from './components/molecules/buttonImportant.vue'
import numberField from './components/molecules/numberField.vue'
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
    <div class="figures-bar">
        <buttonImportant
            class="figures-btn"
            v-for="figure in figures"
            :key="figure.name"
            @click.prevent="console.log(figure.name)"
        >
            <div class="figures-btn-content">
                <img :src="figure.img" alt="circle" />
                <span>{{ figure.name }}</span>
            </div>
        </buttonImportant>
    </div>

    <div class="canvas">a</div>

    <div class="params-box">
        <div class="common-box">
            <h3>Common</h3>
            <numberField placeholder="- - -" />
        </div>
        <div class="specific-box">a</div>
        <div class="submit-box">a</div>
    </div>
</template>

<style scoped>
.figures-bar {
    width: fit-content;
    margin: auto;
    margin-top: 3vh;
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

.canvas {
    width: 80%;
    height: 40vh;
    background-color: white;
    margin: 2rem auto;
}
.params-box {
    width: 100%;
    min-height: 25vh;
    display: flex;
    justify-content: space-around;
}

.params-box > div {
    height: inherit;
    padding: 0 3ch;
}

.params-box > .common-box {
    border-right: 5px dashed brown;
    width: 25%;
}
.params-box > .specific-box {
    width: 50%;
}
.params-box > .submit-box {
    border-left: 5px dashed brown;
    width: 25%;
}
</style>
