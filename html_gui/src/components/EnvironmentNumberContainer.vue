<template>
    <span
        :style="{minWidth: forcedWidth}"
        class="cr-number-holder"
    >
        {{ formattedValue }}
    </span>
</template>

<script>
    export default {
        name: "EnvironmentNumberContainer",

        props: {
            value: {
                type: Number,
                required: true
            },

            base: {
                type: Number,
                default: 10,

                validator: value => value > 0,
            },

            lengthInBytes: {
                type: Number,
                default: 1,

                validator: value => [1, 2].includes(value),
            }
        },

        data() {
            return {
                watchedValue: this.value
            };
        },

        computed: {
            formattedValue() {
                let strMaxLength = (2 ** (this.lengthInBytes * 8))
                    .toString(this.base)
                    .length - 1;

                return this.watchedValue.toString(this.base).padStart(strMaxLength, "0");
            },
        },

        methods: {
            forcedWidth() {
                return this.lengthInBytes + "em";
            }
        },

        watch: {
            value(val) {
                this.watchedValue = val;
            }
        }
    }
</script>


<style lang="less" scoped>
    .cr-number-holder {
        text-align: center;
        font-family: monospace;
    }
</style>
