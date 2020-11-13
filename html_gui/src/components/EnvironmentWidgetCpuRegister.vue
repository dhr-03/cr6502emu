<template>
    <div class="crl-reg-container uk-grid uk-grid-collapse uk-child-width-1-2 uk-text-center">
        <div class="crl-reg-name uk-text-bolder">{{ watchedName }}</div>

        <slot>
            <div>
                <span>{{ watchedPreValue }}</span>

                <EnvironmentNumberContainer
                    :value="watchedValue"
                    :length-in-bytes="lengthInBytes"

                    :base="preferredNumericBase"
                />
            </div>
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

            preValue: {
                type: String,
                default: ""
            },

            value: {
                type: [Number, String],
                required: false,
            },

            lengthInBytes: {
                type: Number,
                required: false,
            }
        },

        data() {
            return {
                watchedName: this.name,
                watchedValue: this.value,
                watchedPreValue: this.preValue,
            };
        },

        watch: {
            name(val) {
                this.watchedName = val;
            },

            value(val) {
                this.watchedValue = val;
            },

            preValue(val) {
                this.watchedPreValue = val;
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
