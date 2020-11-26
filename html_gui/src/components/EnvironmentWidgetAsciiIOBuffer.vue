<template>
    <div class="crl-widget">
        <div>
            <pre class="crl-output-text" ref="scroll">{{ formattedOutput }}</pre>
        </div>

        <div class="crl-input-container uk-grid">
            <div class="uk-width-expand">
                <input
                    type="text" class="crl-input" :placeholder="$t('environment.widget.asciiBuffer.writeHere')"
                >
            </div>

            <div class="uk-width-auto uk-padding-remove">
                <input type="checkbox" v-model="doAutoScroll">
                <font-awesome-icon icon="sort-amount-down-alt"/>
            </div>
        </div>
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
                    let element = this.$refs.scroll;

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
                let element = this.$refs.scroll;

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

    .crl-output-container {
        max-width: 100%;
    }

    .crl-output-text {
        white-space: pre-wrap;
        word-break: break-all;

        margin-bottom: 0;
        padding: 0.2em;

        height: 10.5em;
        overflow-y: auto;

        font-size: 90%;
        font-weight: bold;

        background: #fff;
        color: black;
    }

    .crl-input-container {
        margin-top: 0.70em;

        color: #0a0c0d;
    }

    .crl-input {
        width: 100%;

        border: unset;
    }

</style>
