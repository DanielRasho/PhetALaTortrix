<template>
    <main>
        <div class="canvas"></div>
        <fieldsContainer
         :fields="fields"
         @changesSubmited="updateFields($event)"
        ></fieldsContainer>
    </main>
</template>

<script setup>
import fieldsContainer from '@/components/organism/fieldsContainer.vue'
import { trunk } from '@/lib/figuresDefaults'
import { ref } from 'vue';

const fields = ref({
    axis: {
        x: {
            min: -5,
            max: 5
        },
        y: {
            min: -5,
            max: 5
        },
    },
    figure: {
        majorRadius: {
            value: 1,
            name: 'Major Radius',
            unit: 'm'
        },
        minorRadius: {
            value: 1,
            name: 'Minus Radius',
            unit: 'm'
        },
        heigh: {
            value: 1,
            name: 'Height',
            unit: 'm'
        },
        charge: {
            value: 1,
            name: 'Charge',
            unit: 'nC'
        },
    },
    points: []
})


function updateFields(newValue) {
    fields.value = newValue
    console.log("FROM PARENT");
    console.dir(fields.value);
    drawCanvas()
}
function drawCanvas(){
    // TODO
}
</script>

<style scoped>
main {
    grid-area: 'main';
    padding: 3ch;
}
.canvas {
    display: block;
    background-color: rgb(207, 207, 207);
    border-radius: 7px;
    min-height: 55vh;
    margin-bottom: 1ch;
}
</style>
