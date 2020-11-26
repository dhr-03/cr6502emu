<template>
    <div class="cr-logbar">
        <div class="cr-logbar-header">

            <div
                class="cr-logbar-toggle"
                uk-toggle="target: .__logbar-toggle; animation: uk-animation-slide-bottom uk-animation-fast"
            >

                <font-awesome-icon
                    class="__logbar-toggle"
                    icon="chevron-right"
                />
                <font-awesome-icon
                    class="__logbar-toggle"
                    icon="chevron-down"
                    hidden
                />

                <span
                    class="uk-text-bold uk-padding-small uk-padding-remove-vertical"
                    v-t="'environment.logbar.title'"
                />
            </div>

            <Badge
                type="err"
                v-if="errMsgCount > 0"

                :value="errMsgCount"
            />
            <Badge
                type="warn"
                v-if="warnMsgCount > 0"

                :value="warnMsgCount"
            />
            <Badge
                type="info"
                v-if="infoMsgCount"

                :value="infoMsgCount"
            />

        </div>

        <div class="cr-logbar-content-container">
            <div class="cr-logbar-content __logbar-toggle" id="logbarContent" hidden>
                <div class="cr-logbar-empty">
                    <div class="cr-logbar-empty-icon">
                        <font-awesome-icon icon="ghost"/>
                    </div>
                    <span v-t="'environment.logbar.emptyMessage'"/>
                </div>

                <Alert
                    v-for="(item, index) in messagesList"
                    :key="index"

                    :type="item.type"
                >
                    <template v-slot:title>{{ item.title }}: </template>

                    <i18n :path="item.templateId" tag="span">

                        <template
                            v-slot:code
                            v-if="item.codeItems.length"
                        >
                            <code class="__code_reset">{{ item.codeItems[0] }}</code>
                        </template>

                        <template
                            v-slot:code2
                            v-if="item.codeItems.length > 1"
                        >
                            <code class="__code_reset">{{ item.codeItems[1] }}</code>
                        </template>
                    </i18n>
                </Alert>
            </div>
        </div>
    </div>
</template>

<script>
    import Badge from "./Badge"

    import {Logger} from "../assets/wasm/shared/logger"
    import Alert from "./Alert"
    import {mapMutations, mapGetters} from "vuex"

    export default {
        name: "EnvironmentLogbar",
        components: {Alert, Badge},

        computed: {
            ...mapGetters("env", [
                "messagesList"
            ]),

            errMsgCount() {
                return this.messagesList.filter(m => m.type === "err").length;
            },

            warnMsgCount() {
                return this.messagesList.filter(m => m.type === "warn").length;
            },

            infoMsgCount() {
                return this.messagesList.filter(m => m.type === "info").length;
            },
        },

        methods: mapMutations("env", [
            "addMessage",
            "resetMessages"
        ]),

        created() {
            Logger.setup(this.addMessage);
        }

    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-logbar {
        background: @oc-gray-9;

        padding: 0.5em 0.5em 0.5em 1em;

        position: fixed;
        left: 0;
        bottom: 0;
        width: 100%;

        z-index: 9999999999;

        overflow-y: scroll;
    }

    .cr-logbar-header {
        user-select: none;
    }

    .cr-logbar-toggle {
        display: inline-block;
    }

    .cr-logbar-empty {
        display: flex;

        width: 100%;
        height: 100%;


        justify-content: center;
        align-items: center;
        flex-direction: column;

        user-select: none;
        opacity: 0.60;

        &:not(:only-child) {
            display: none;
        }
    }

    .cr-logbar-empty-icon {
        font-size: 250%;
        margin-bottom: 0.2em
    }

    .cr-logbar-content-container {
        margin-right: 0.7em;
    }

    .cr-logbar-content {
        margin-top: 0.5em;

        padding-right: 2em;
        margin-right: 0.5em;
        height: 40vh;
        overflow-y: scroll;

        &::-webkit-scrollbar {
            width: 0.5em;
        }

        &::-webkit-scrollbar-track {
            background: #2b8a3e
        }
    }

    .cr-alert-part {
        margin-right: 0.3em;
    }

    .__code_reset {
        font-family: Consolas, monaco, monospace;
        font-size: 0.875rem;
        color: rgba(255, 255, 255, .7);
        background: rgba(255, 255, 255, .1);
        white-space: nowrap;
        padding: 2px 6px;
    }

</style>
