<template>
    <nav
        class="crl-actionbar uk-navbar uk-navbar-container uk-navbar-transparent"
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
                        :on-click="onStepShort"
                    />

                    <EnvironmentActionbarButton
                        icon="chevron-right"
                        explanation="Execute Instruction"
                        color-name="yellow"

                        :enabled="ableToStep"
                        :on-click="onStepLong"
                    >
                        <font-awesome-icon icon="chevron-right"/>
                    </EnvironmentActionbarButton>

                </ul>
            </div>

            <div class="uk-width-auto">
                <ul class="uk-navbar-nav uk-text-left">

                    <Modal
                        :esc-close="false"
                        :bg-close="false"

                        :show-header="false"
                        :show-footer="true"

                        :container="true"
                        :center="true"

                        close-button-type="none"

                        dom-id="settingsMenu"

                        content-class="crl-settings-menu"

                        ref="modal"
                    >
                        <template v-slot:toggle>
                            <EnvironmentActionbarButton
                                icon="cog"
                                explanation="Settings"
                                color-name="gray"

                                :enabled="ableToConfig"
                            >
                                <b> Settings</b>
                            </EnvironmentActionbarButton>
                        </template>

                        <template v-slot:body>
                            <EnvironmentSettingsMenu class="crl-settings-container"/>
                        </template>

                        <template v-slot:footer>
                            <button
                                @click="saveChanges"

                                class="uk-button uk-button-primary"
                            >
                                Save
                            </button>
                        </template>

                    </Modal>
                </ul>
            </div>

        </div>
    </nav>
</template>

<script>
    import EnvironmentActionbarButton from "./EnvironmentActionbarButton"
    import EnvironmentActionbarSeparator from "./EnvironmentActionbarSeparator"
    import {mapGetters, mapActions} from "vuex"
    import Modal from "./Modal";
    import Environment from "../views/Environment";
    import EnvironmentSettingsMenu from "./EnvironmentSettingsMenu";

    export default {
        name: "EnvironmentActionbar",
        components: {
            EnvironmentSettingsMenu,
            Environment,
            Modal,
            EnvironmentActionbarSeparator,
            EnvironmentActionbarButton
        },

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
                "resetSystem",
                "toggleRun",
                "toggleDebug",
                "systemTick",
                "systemExecuteOperation",
            ]),

            ...mapActions("prj", [
                "scheduleCurrentProjectSave",
            ]),

            onBuild() {
                this.buildToRom();
            },

            onReset() {
                this.resetSystem();
            },

            onRunToggled() {
                this.toggleRun();
            },

            onDebugToggled() {
                this.toggleDebug();
            },

            onStepShort() {
                this.systemTick();
            },

            onStepLong() {
                this.systemExecuteOperation();
            },

            saveChanges() {
                this.$refs.modal.hideModal();

                this.scheduleCurrentProjectSave();
            }

        },
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .crl-actionbar {
        background: @oc-gray-9;

        padding-left: 3em;
        padding-right: 3em;

        margin-bottom: 2em;
    }

</style>

<style lang="less">
    @import "../../node_modules/open-color/open-color";

    .crl-settings-menu {
        .uk-modal-body {
            height: 100vh; //let uikit cap it to the max value
            overflow: hidden;
            padding: 0;
        }
    }


</style>
