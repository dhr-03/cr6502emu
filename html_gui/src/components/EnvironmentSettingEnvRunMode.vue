<template>
    <div class="uk-form-stacked">

        <p v-t="'environment.settings.EnvironmentSettingEnvRunMode.explanation.a'"/>
        <p v-t="'environment.settings.EnvironmentSettingEnvRunMode.explanation.b'"/>
        <p v-t="'environment.settings.EnvironmentSettingEnvRunMode.explanation.c'"/>

        <hr>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingEnvRunMode.operationsPerCycle'"
            />
            <input
                v-model.number="operationsPerCycle"

                required

                class="uk-input"
                type="number"
                placeholder="Operations per cycle"
            >
        </div>
    </div>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import {ProjectSchema} from "../assets/schema/project";

    const MinOperations = ProjectSchema.properties.settings.properties.runModeOperationsPerCycle.minimum;
    const MaxOperations = ProjectSchema.properties.settings.properties.runModeOperationsPerCycle.maximum;

    export default {
        name: "EnvironmentSettingEnvRunMode",
        mixins: [MixinSettingsPage],

        computed: {
            operationsPerCycle: {
                get() {
                    return this.projectSettings.runModeOperationsPerCycle;
                },

                set(val) {
                    let validValue = Math.min(
                        MaxOperations, Math.max(MinOperations, val)
                    );

                    this.projectSettings.runModeOperationsPerCycle = validValue;
                }
            },
        }
    }
</script>
