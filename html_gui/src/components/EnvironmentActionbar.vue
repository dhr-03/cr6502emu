<template>
    <nav
            class="cr-actionbar uk-navbar uk-navbar-container uk-navbar-transparent"
            uk-sticky
    >

        <div class="uk-grid uk-grid-large uk-width-expand">

            <div class="uk-width-auto">
                <ul class="uk-navbar-nav">

                    <EnvironmentActionbarButton
                            icon="hammer"
                            explanation="Build"

                            :enabled="ableToBuild"
                            :on-click="onBuild"
                    />

                    <EnvironmentActionbarSeparator/>

                    <EnvironmentActionbarButton
                            icon="sync-alt"
                            explanation="Reset"

                            :enabled="ableToReset"
                            :on-click="onReset"
                    />

                </ul>
            </div>

            <div class="uk-width-expand uk-flex uk-flex-center">
                <ul class="uk-navbar-nav">
                    <EnvironmentActionbarButton
                            icon="play"
                            icon-active="stop"
                            color-name-active="red"
                            explanation="Run"

                            :enabled="ableToRun"
                            :active="isRunning"
                            :on-click="onRunToggled"
                    />

                    <EnvironmentActionbarSeparator/>

                    <EnvironmentActionbarButton
                            icon="bug"
                            icon-active="stop"
                            color-name-active="red"
                            explanation="Debug"

                            :enabled="ableToDebug"
                            :active="isDebugging"
                            :on-click="onDebugToggled"
                    />

                    <EnvironmentActionbarButton
                            icon="chevron-right"
                            explanation="Execute Cycle"
                            color-name="yellow"

                            :enabled="ableToStep"
                    />

                    <EnvironmentActionbarButton
                            icon="chevron-right"
                            explanation="Execute Instruction"
                            color-name="yellow"

                            :enabled="ableToStep"
                    >
                        <font-awesome-icon icon="chevron-right"/>
                    </EnvironmentActionbarButton>

                </ul>
            </div>

            <div class="uk-width-auto">
                <ul class="uk-navbar-nav uk-text-left">
                    <EnvironmentActionbarButton
                            icon="cog"
                            explanation="Settings"
                            color-name="gray"

                            :enabled="ableToConfig"
                    >
                        <b> Settings</b>
                    </EnvironmentActionbarButton>
                </ul>
            </div>

        </div>
    </nav>
</template>

<script>
    import EnvironmentActionbarButton from "./EnvironmentActionbarButton";
    import EnvironmentActionbarSeparator from "./EnvironmentActionbarSeparator";
    import {mapGetters, mapActions} from "vuex";

    export default {
        name: "EnvironmentActionbar",
        components: {EnvironmentActionbarSeparator, EnvironmentActionbarButton},

        computed: mapGetters("env", [
            "isRunning",
            "isDebugging",

            "ableToBuild",
            "ableToReset",
            "ableToConfig",
            "ableToRun",
            "ableToDebug",
            "ableToStep",
            "ableToConfig",
        ]),

        methods: {
            ...mapActions("env", [
                "buildToRom",
                "toggleRun",
                "toggleDebug",
            ]),

            onBuild() {
                this.$emit("env_build")
            },

            onReset() {
              this.$emit("env_reset");
            },

            onRunToggled() {
                this.$emit("env_toggle-run")
            },


            onDebugToggled() {
                this.$emit("env_toggle-debug")
            },
        },
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-actionbar {
        background: @oc-gray-9;

        padding-left: 3em;
        padding-right: 3em;

        margin-bottom: 2em;
    }
</style>
