<template>
    <div class="crl-inherit-mh uk-grid uk-grid-collapse uk-light">
        <div class="crl-inherit-mh crl-scrollable uk-width-1-5">

            <div
                v-for="(component, index) of settingsComponents"
                :key="index"

                class="crl-submenu uk-text-truncate"
                :class="{'selected': component === activeComponent}"

                @click="updateActiveComponent(component)"
            >
                {{ $t(getNiceNameKeyForComponent(component)) }}
            </div>

        </div>

        <div class="crl-setting-container crl-inherit-mh crl-scrollable uk-width-expand">
            <h3 v-t="activeComponentNiceNameKey"/>
            <hr>

            <component :is="activeComponent"/>
        </div>
    </div>
</template>

<script>
    import EnvironmentSettingPrjMeta from "./EnvironmentSettingPrjMeta";
    import EnvironmentSettingPrjFile from "./EnvironmentSettingPrjFile";
    import EnvironmentSettingEnvInterface from "./EnvironmentSettingEnvInterface";
    import EnvironmentSettingPrjDevices from "./EnvironmentSettingPrjDevices";
    import EnvironmentSettingEnvRunMode from "./EnvironmentSettingEnvRunMode";

    export default {
        name: "EnvironmentSettingsMenu",
        components: {
            EnvironmentSettingPrjMeta,
            EnvironmentSettingPrjFile,
            EnvironmentSettingEnvInterface,
            EnvironmentSettingPrjDevices,
            EnvironmentSettingEnvRunMode,
        },

        data() {
            return {
                settingsComponents: [
                    "EnvironmentSettingPrjMeta",
                    "EnvironmentSettingPrjFile",
                    "EnvironmentSettingEnvInterface",
                    "EnvironmentSettingPrjDevices",
                    "EnvironmentSettingEnvRunMode",
                ],

                activeComponent: "EnvironmentSettingPrjMeta",
            }
        },

        methods: {
            updateActiveComponent(to) {
                if (this.activeComponent !== to) {
                    this.activeComponent = to;
                }
            },

            getNiceNameKeyForComponent(name) {
                return `environment.settings.${name}.niceName`;
            }
        },

        computed: {
            activeComponentNiceNameKey() {
                return this.getNiceNameKeyForComponent(this.activeComponent);
            }
        }
    }
</script>

<style lang="less" scoped>
    .crl-inherit-mh {
        max-height: inherit;
    }

    .crl-scrollable {
        overflow: auto;
    }

    .crl-submenu {
        padding: 0.5em;

        border-top: 1px solid #555;
        border-bottom: 1px solid #555;

        &.selected {
            color: white;
            font-weight: bold;
        }
    }

    .crl-setting-container {
        padding: 1em;
    }

</style>
