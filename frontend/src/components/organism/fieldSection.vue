<template>
    <div>
        <h3>{{ title }}</h3>
        <slot></slot>
    </div>
</template>

<script setup>

const props = defineProps({
    title : {
        required: true,
        type: String,
        default: "Title"
    }
})

</script>

<style scoped>

</style>