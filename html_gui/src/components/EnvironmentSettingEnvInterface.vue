<template>
    <form class="uk-form-stacked">

        <div class="uk-margin">
            <label class="uk-form-label">Preferred numeric base</label>

            <select
                v-model.number="projectSettings.preferredNumericBase"

                class="uk-select"
            >

                <option value="10">Decimal (10)</option>
                <option value="16">Hexadecimal (16)</option>

            </select>
        </div>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Maximum Memory Monitor rows <b>(This can impact performance and loading times)</b></label>
            <input
                v-model.number="memMonitorMaxRows"

                class="uk-input"
                type="number"

                min="1"
                max="500"
            >
        </div>

    </form>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import {ProjectSchema} from "../assets/schema/project";

    const MEMORY_MONITOR_MIN_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.minimum;
    const MEMORY_MONITOR_MAX_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.maximum;

    export default {
        name: "EnvironmentSettingPrjMeta",
        niceName: "Interface",
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
