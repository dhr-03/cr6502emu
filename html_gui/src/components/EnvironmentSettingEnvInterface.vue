<template>
    <div class="uk-form-stacked">

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingEnvInterface.preferredNumBase'"
            />

            <select
                v-model.number="projectSettings.preferredNumericBase"

                class="uk-select"
            >

                <option value="10">
                    {{ $t("guiCommon.numericBase.dec") }}
                </option>

                <option value="16">
                    {{ $t("guiCommon.numericBase.hex") }}
                </option>

            </select>
        </div>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">
                <span
                    v-t="'environment.settings.EnvironmentSettingEnvInterface.maxMonitorRows'"
                />
                <b
                    class="cr-mg-t"
                    v-t="'environment.settings.EnvironmentSettingEnvInterface.maxMonitorRowsExplanation'"
                />
            </label>

            <input
                v-model.number="memMonitorMaxRows"

                class="uk-input"
                type="number"

                min="1"
                max="500"
            >
        </div>

    </div>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import {ProjectSchema} from "../assets/schema/project";

    const MEMORY_MONITOR_MIN_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.minimum;
    const MEMORY_MONITOR_MAX_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.maximum;

    export default {
        name: "EnvironmentSettingEnvInterface",
        mixins: [MixinSettingsPage],

        computed: {
            memMonitorMaxRows: {
                get() {
                    return this.projectSettings.memoryMonitorMaxRows;
                },

                set(value) {
                    let validValue = Math.min(MEMORY_MONITOR_MAX_ROWS,
                        Math.max(MEMORY_MONITOR_MIN_ROWS, value)
                    );

                    this.projectSettings.memoryMonitorMaxRows = validValue;
                }
            }
        }
    }
</script>
