<template>
    <div class="crl-widget">
        <div
            ref="a"
            class="crl-output"

            @scroll="onOutputScroll"
        >
            <pre>{{ formattedOutput }}</pre>
        </div>

        <input
            @keypress="onInput"
            type="text" class="crl-input" placeholder="Write here ..."
        >
    </div>
</template>

<script>
    import MixinEnvironmentWidget from "./MixinEnvironmentWidget";

    export default {
        name: "EnvironmentWidgetAsciiIOBuffer",
        mixins: [MixinEnvironmentWidget],

        data() {
            return {
                doAutoScroll: true,
            };
        },

        computed: {
            formattedOutput() {
                return this.widget.displayData.out.join("");
            },
        },

        watch: {
            formattedOutput() {
                if (this.doAutoScroll) {
                    let element = this.$refs.a;

                    element.scrollTop = element.scrollHeight;
                }
            }
        },

        methods: {
            onInput(e) {
                this.widget.displayData.in.push(e.keyCode);

                e.preventDefault();
            },

            onOutputScroll() {
                let element = this.$refs.a;

                let elementHeight = element.getBoundingClientRect().height;

                this.doAutoScroll = (element.scrollTop + elementHeight) >= element.scrollHeight;
            }
        }
    }
</script>

<style lang="less" scoped>
    .crl-widget {
        margin: 0.5em;
    }

    .crl-output {
        max-width: 100%;

        padding: 0.1em;

        background: #fff;

        height: 10.5em;
        overflow-y: auto;

        word-break: break-all;

        font-size: 90%;
        font-weight: bold;

        color: black;
    }

    .crl-input {
        margin-top: 1em;

        width: 100%;

        border: unset;
    }

</style>
