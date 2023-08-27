<template>
    <main>
        <canvas></canvas>
        <fieldsContainer></fieldsContainer>
    </main>
</template>

<script setup>
import fieldsContainer from '@/components/organism/fieldsContainer.vue'
import { ref } from 'vue'

const fields = ref({
    axis: {
        x: [-5, 5],
        y: [-5, 5]
    },
    figure: {
        radius: {
            value: 32,
            name: 'Radius',
            unit: 'm'
        },
        Charge: 34
    },
    points: []
})
</script>
<style scoped>
main {
    flex-grow: 1;
    padding: 3ch;
}
.canvas {
    display: block;
    background-color: rgb(207, 207, 207);
    border-radius: 7px;
    min-height: 55vh;
    margin-bottom: 3ch;
}
</style>
