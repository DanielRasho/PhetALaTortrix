<template>
    <main>
        <div class="canvas"></div>
        <fieldsContainer></fieldsContainer>
    </main>
</template>

<script setup>
import fieldsContainer from '@/components/organism/fieldsContainer.vue'
</script>

<style scoped>
main {
    grid-area: 'main';
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
