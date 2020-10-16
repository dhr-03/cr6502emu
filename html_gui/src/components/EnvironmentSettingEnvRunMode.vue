<template>
    <div class="uk-form-stacked">

        <p>
            Due to some technical difficulties, the project can only execute ASM code every 4ms-10ms (approx).
        </p>

        <p>
            The following setting determines how many opcodes are executed at once in every cycle (execute and update
            widgets).
        </p>

        <p>
            Be aware that while the cycle is running the whole web page will freeze.
        </p>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Operations per cycle</label>
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

        niceName: "Run Mode",

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

                    console.log(this.projectSettings.runModeOperationsPerCycle)
                }
            },

            a() {
                return this.projectSettings.runModeOperationsPerCycle
            }
        }
    }
</script>
