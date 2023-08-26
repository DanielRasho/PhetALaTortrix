<script setup>
import buttonImportant from './components/molecules/buttonImportant.vue'
import numberField from './components/molecules/numberField.vue'
import { ref } from 'vue'

const figures = ref([
    {
        name: 'Cone',
        img: 'circle.png'
    },
    {
        name: 'Truncated Cone',
        img: 'circle.png'
    },
    {
        name: 'Semiesphere',
        img: 'circle.png'
    }
])
</script>

<template>
    <div class="figure-display">
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
    </div>

    <div class="params-box">
        <div class="common-box">
            <h3>Common</h3>
            <numberField name="Carga" placeholder="- - -" unit="nC" />
        </div>
        <div class="specific-box">a</div>
        <div class="submit-box">a</div>
    </div>
</template>

<style scoped>

.figure-display{
   display: flex; 
   padding-top: 3vh;
   padding: 3ch 3ch 3ch;
}
.figures-bar {
    width: fit-content;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-right: 3ch;
}

.figures-btn {
    margin: 1.5ch 0;
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

.canvas {
    flex-grow: 1;
    background-color: rgb(207, 207, 207);
    border-radius: 7px;
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
    border-right: 1px solid #ccc;
    width: 25%;
}
.params-box > .specific-box {
    width: 50%;
}
.params-box > .submit-box {
    border-left: 1px solid #ccc;
    width: 25%;
}
</style>
