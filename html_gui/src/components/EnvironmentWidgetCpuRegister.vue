<template>
    <div class="crl-reg-container uk-grid uk-grid-collapse uk-child-width-1-2 uk-text-center">
        <div class="crl-reg-name uk-text-bolder">{{ name }}</div>

        <slot>
            <EnvironmentNumberContainer
                :value="watchedValue"
                :length-in-bytes="lengthInBytes"

                :base="preferredNumericBase"
            />
        </slot>
    </div>
</template>

<script>
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";

    export default {
        name: "EnvironmentWidgetCpuRegister",
        mixins: [MixinPreferredNumericBase],
        components: {EnvironmentNumberContainer},

        props: {
            name: {
                type: String,
                required: true,
            },

            value: {
                type: [Number, String],
                required: true,
            },

            lengthInBytes: {
                type: Number,
                required: false,
            }
        },

        data() {
            return {
                watchedValue: this.value
            };
        },

        watch: {
            value(val) {
                this.watchedValue = val;
            }
        }
    }
</script>

<style lang="less" scoped>

    .crl-reg-container {
        font-size: 75%;
    }

    .crl-reg-name:after {
        float: right;
        content: "\27f6";
    }
</style>
