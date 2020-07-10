<template>
    <div
            class="cr-badge"
            :class="typeClass"
    >
        <slot>
            {{ cappedValue }}
        </slot>
    </div>
</template>

<script>

    import MixinStatusElement from "./MixinStatusElement";

    export default {
        name: "Badge",

        mixins: [MixinStatusElement],

        props: {
            value: {
                type: Number,
                default: 0
            }
        },

        data() {
            return {
                dataValue: this.value
            };
        },

        watch: {
            value(val) {
                this.dataValue = val;
            }
        },

        computed: {
            cappedValue() {
                return this.dataValue < 10 ? this.dataValue : "9+";
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/modifierStyles";

    .cr-badge {
        display: inline-block;
        vertical-align: middle;

        border-radius: 0.7em;
        padding: 0em 0.5em;
        font-weight: bold;

        min-width: 1.2em;
        text-align: center;

        margin-right: 0.8em;
    }
</style>
