<template>
    <div class="container">
        <fieldSection title="Axis" class="common-box">
            <h4>X axis</h4>
            <div class="wrapper-container">
                <numberField
                    name="Min"
                    placeholder="- - -"
                    unit="m"
                    width="7ch"
                    :initialValue="fields.axis.x[0]"
                    @field-updated="fields.axis.x[0] = $event"
                />
                <numberField
                    name="Max"
                    placeholder="- - -"
                    unit="m"
                    width="7ch"
                    :initialValue="fields.axis.x[1]"
                    @field-updated="fields.axis.x[1] = $event"
                />
            </div>
            <h4>Y axis</h4>
            <div class="wrapper-container">
                <numberField
                    name="Min"
                    placeholder="- - -"
                    unit="m"
                    width="7ch"
                    :initialValue="fields.axis.y[0]"
                    @field-updated="fields.axis.y[0] = $event"
                />
                <numberField
                    name="Max"
                    placeholder="- - -"
                    unit="m"
                    width="7ch"
                    :initialValue="fields.axis.y[1]"
                    @field-updated="fields.axis.y[1] = $event"
                />
            </div>
        </fieldSection>

        <fieldSection title="Figure" class="specific-box">
        <div class="wrapper-container">
            <numberField v-for="field in fields.figure" :key="field.name"
            class="specific-field"
            :name="field.name"
            :unit="field.unit"
            :titleWidth="calculateFieldNameWidth(fields)"
            :initialValue="field.value"
            @field-updated="field.value = $event"
            />
        </div>
        </fieldSection>

        <fieldSection title="Point" class="submit-box">
            <numberField
                name="Position"
                unit="m"
                width="10ch"
            />
            <div class="wrapper-container-center">
                <buttonImportant
                    class="submit-btn"
                    @click="console.log(fields)"
                >
                    Submit <i class="fa-solid fa-arrow-right"></i>
                </buttonImportant>
                <buttonImportant class="clear-btn"
                    >Clear <i class="fa-solid fa-trash"></i
                ></buttonImportant>
            </div>
        </fieldSection>
    </div>
</template>

<script setup>
import buttonImportant from '@/components/atoms/buttonImportant.vue'
import numberField from '@/components/atoms/numberField.vue'
import fieldSection from '@/components/molecules/fieldSection.vue'
import { onMounted,  ref } from 'vue'

const fields = ref({
    axis: {
        x: [-5, 5],
        y: [-5, 5]
    },
    figure: {
        radius: {
            value: 1,
            name: 'Radius',
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

function calculateFieldNameWidth(names){
    let maxLen = 0;
    for (let [key, proxy] of Object.entries(names.figure)) {
        let proxyLen = proxy.name.length
        if(proxyLen > maxLen)
            maxLen = proxyLen
    }
    return maxLen + 'ch';
}

</script>

<style scoped>
h4 {
    font-size: 1.5rem;
    color: #b1b1b1;
    margin: 1ch 0;
}
.container {
    width: 100%;
    display: flex;
    justify-content: space-around;
    margin-top: 2rem;
}

.container > .common-box {
    border-right: 1px solid #ccc;
    width: 25%;
    padding-right: 2ch;
}
.container > .specific-box {
    width: 50%;
    padding: 0 2ch;
}
.container > .submit-box {
    border-left: 1px solid #ccc;
    width: 25%;
    padding-left: 2ch;
}

.specific-field{
    margin-top: 2ch;
}
.wrapper-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
}


.wrapper-container-center {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}

.submit-btn,
.clear-btn {
    display: block;
    margin: 2rem auto 0;
}

.submit-btn :deep(.front) {
    background-color: rgb(117, 208, 117);
    border: 7px solid rgb(60, 106, 60);
    color: rgb(35, 60, 35);
    width: 10ch;
    font-size: 1.5rem;
}
.submit-btn :deep(.front) i {
    color: rgb(35, 60, 35);
}
.submit-btn :deep(.edge) {
    background-color: rgb(35, 60, 35);
}
.clear-btn :deep(.front) {
    background-color: rgb(208, 137, 117);
    border: 7px solid rgb(131, 35, 9);
    color: rgb(131, 35, 9);
    width: 10ch;
    font-size: 1.5rem;
}
.clear-btn :deep(.front) i {
    color: rgb(131, 35, 9);
}
.clear-btn :deep(.edge) {
    background-color: rgb(78, 24, 1);
}
</style>
