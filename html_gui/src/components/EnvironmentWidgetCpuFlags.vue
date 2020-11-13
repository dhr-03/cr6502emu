<template>
    <div>
        <span class="crl-flag" :uk-tooltip="$t('environment.widget.cpu.flags.negative')">{{ formattedFlags.n }}</span>
        <span class="crl-flag" :uk-tooltip="$t('environment.widget.cpu.flags.overflow')">{{ formattedFlags.o }}</span>
        <span class="crl-flag" :uk-tooltip="$t('environment.widget.cpu.flags.zero')">{{ formattedFlags.z }}</span>
        <span class="crl-flag" :uk-tooltip="$t('environment.widget.cpu.flags.carry')">{{ formattedFlags.c }}</span>
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
                    n: getFlag((this.value >> 7) & 0b1, "n"),
                    o: getFlag((this.value >> 6) & 0b1, "o"),
                    z: getFlag((this.value >> 1) & 0b1, "z"),
                    c: getFlag((this.value >> 0) & 0b1, "c"),
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
