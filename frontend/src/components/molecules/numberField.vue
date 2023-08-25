<template>
    <div>
        <span>{{ this.name }}</span>
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
input,
.unit {
    display: inline-block;
    height: 2rem;
    padding: 2ch 2ch;
    font-family: 'PT Sans', sans-serif;
    font-size: 1rem;
}

input {
    border: 2px solid #ccc;
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
    border-radius: 0px 7px 7px 0px;
}
</style>
