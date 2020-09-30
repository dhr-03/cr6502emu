<template>
    <input
        v-model="rawValue"
        @input="onInput"

        type="text"

        class="uk-input"
        :class="{'crl-invalid-input': !valueIsValid}"
    >
</template>

<script>
    import Tools from "../assets/js/tools";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";

    export default {
        name: "NumericInput",
        mixins: [MixinPreferredNumericBase],

        props: {
            initialValue: {
                type: [String, Number],
                default: "0",
            },

            numericBase: {
                type: Number,
                default: null,
            },

            minValue: {
                type: Number,
                default: 0
            },

            maxValue: {
                type: Number,
                default: Tools.U16MaxValue,
            },
        },

        data() {
            return {
                rawValue: this.initialValue,
                numberValue: Number(this.initialValue),

                watchedNumericBase: this.numericBase,
            };
        },

        computed: {
            valueIsValid() {
                return this.numberValue !== null;
            },
        },

        methods: {
            onInput() {
                let prefix = this.watchedNumericBase === 16 ? "0x" : "";
                let parsedValue = Number(`${prefix}${this.rawValue}`);

                let isValid = !isNaN(parsedValue) &&
                    parsedValue >= this.minValue && parsedValue <= this.maxValue;


                let validatedValue = isValid ? parsedValue : null;

                this.numberValue = validatedValue
                this.$emit("value-changed", validatedValue);
            }
        },

        watch: {
            numericBase(newVal) {
                this.watchedNumericBase = newVal;

                this.onInput();
            },
        },

        created() {
            if (this.watchedNumericBase == null) {
                this.watchedNumericBase = this.preferredNumericBase;
            }

            this.onInput();
        }

    }
</script>

<style lang="less" scoped>
    .crl-invalid-input {
        border: 1px solid red;
    }
</style>
