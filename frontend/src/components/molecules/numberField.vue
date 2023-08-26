<template>
    <div class="field-container">
        <span class="title">{{ this.name }}</span>
        <input
            @input="updateValue"
            type="text"
            :placeholder="this.placeholder"
        />
        <span class="unit">{{ this.unit }}</span>
    </div>
</template>

<script>
/**
 * Representation of a form's field. Code to works with v-model
 */
export default {
    props: {
        /**
         * Required for v-model behaviour.
         */
        modelValue: {},
        name: {
            required: true,
            type: String,
            default: 'Title'
        },

        unit: {
            required: true,
            type: String,
            default: '?'
        },
        /**
         * Renders the placeholder value for some input types.
         */
        placeholder: {
            required: false,
            type: String,
            default: 'Insert a value'
        }
    },

    emits: ['update:modelValue'],

    data() {
        return {
            currentValue: null
        }
    },
    methods: {
        /**
         * Call when the input's value is modify.
         * @param {event} event Action of changing INPUT's value.
         */
        updateValue(event) {
            // Update value
            this.currentValue = event.target.value
            // Emit the value to parent
            this.$emit('update:modelValue', this.currentValue)
        }
    }
}
</script>

<style scoped>

.field-container{
    display: flex;
    align-items: center;
}

.title {
    font-size: 1.2rem;
    margin-right: 2ch;
}

input,
.unit {
    display: inline-block;
    border: 2px solid #ccc;
    height: 2.5rem;
    padding: 1ch 1ch;
    font-family: 'PT Sans', sans-serif;
    font-size: 1rem;
}

input {
    border-right: 0;
    width: 20ch;
    color: #666;
    border-radius: 7px 0px 0px 7px;
}
input::placeholder {
    font-style: italic;
}

.unit {
    background-color: #ccc;
    color: var(--background);
    border-radius: 0px 7px 7px 0px;
    font-weight: bold;
}
</style>
