<template>
    <span>{{ formattedFlags }}</span>
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
                const FLAGS = "cz___-on";

                let formatted = "";

                for (let i = 0; i < FLAGS.length; i++) {
                    if ((this.value >> i) & 0b1) {
                        formatted += FLAGS[i].toUpperCase();
                    } else {
                        formatted += FLAGS[i];
                    }
                }

                return formatted;
            }
        },

        watch: {
            value(val) {
                this.watchedValue = val;
            }
        }
    }
</script>
