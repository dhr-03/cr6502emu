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
                {{ getNiceNameForComponent(component) }}
            </div>

        </div>

        <div class="crl-setting-container crl-inherit-mh crl-scrollable uk-width-expand">
            <h3>{{ activeComponentNiceName }}</h3>
            <hr>

            <component :is="activeComponent"/>
        </div>
    </div>
</template>

<script>
    import EnvironmentSettingPrjMeta from "./EnvironmentSettingPrjMeta";
    import EnvironmentSettingPrjFile from "./EnvironmentSettingPrjFile";
    import EnvironmentSettingEnvInterface from "./EnvironmentSettingEnvInterface";

    export default {
        name: "EnvironmentSettings",
        components: {EnvironmentSettingPrjMeta, EnvironmentSettingPrjFile, EnvironmentSettingEnvInterface},

        data() {
            return {
                settingsComponents: [
                    "EnvironmentSettingPrjMeta",
                    "EnvironmentSettingEnvInterface",
                    "EnvironmentSettingPrjFile",
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

            getNiceNameForComponent(name) {
                return this.$options.components[name].niceName;
            }
        },

        computed: {
            activeComponentNiceName() {
                return this.getNiceNameForComponent(this.activeComponent);
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
