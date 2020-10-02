<template>
    <div>
        <span class="crl-flag" uk-tooltip="Negative">{{ formattedFlags.n }}</span>
        <span class="crl-flag" uk-tooltip="Overflow">{{ formattedFlags.o }}</span>
        <span class="crl-flag" uk-tooltip="Zero">{{ formattedFlags.z }}</span>
        <span class="crl-flag" uk-tooltip="Carry">{{ formattedFlags.c }}</span>
    </div>
</template>

<script>
    export default {
        name: "EnvironmentWidgetCpuFlags",

        props: {
            value: {
                type: Number,
                required: true,

                validator: v => v >= 0 && v <= 0xFF,
            }
        },

        data() {
            return {
                watchedValue: this.value,
            };
        },

        computed: {
            formattedFlags() {
                function getFlag(value, name) {
                    return value ? name.toUpperCase() : name;
                }

                return {
                    n: getFlag(this.value >> 7, "n"),
                    o: getFlag(this.value >> 6, "o"),
                    z: getFlag(this.value >> 1, "z"),
                    c: getFlag(this.value >> 0, "c"),
                };
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
    .crl-flag {
        &:not(:last-child) {
            margin-right: 0.5em;
        }
    }
</style>
