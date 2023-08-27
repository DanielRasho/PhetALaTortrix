<template>
    <div class="container">
        <div class="params-box">
            <fieldSection title="Este es un titulo" class="common-box">
                <numberField name="Carga" placeholder="- - -" unit="nC" />
            </fieldSection>
            <fieldSection title="Este es un titulo" class="specific-box">
                <numberField name="Carga" placeholder="- - -" unit="nC" />
            </fieldSection>
            <fieldSection title="Este es un titulo" class="submit-box">
                <buttonImportant> ara ara </buttonImportant>
            </fieldSection>
        </div>
    </div>
</template>

<script setup>
import buttonImportant from '@/components/atoms/buttonImportant.vue'
import numberField from '@/components/atoms/numberField.vue'
import fieldSection from '@/components/molecules/fieldSection.vue'
</script>

<style scoped>
.params-box {
    width: 100%;
    min-height: 30vh;
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
