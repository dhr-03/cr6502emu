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
                class="uk-input"
                type="number"

                min="1"
                max="500"

                :value="this.projectSettings.memoryMonitorMaxRows"
                ref="memMonRowsInput"
                @focusout="syncMemMonitorRowsLimit"
            >
        </div>

    </div>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import {ProjectSchema} from "../assets/schema/project";

    import UIkit from "uikit";

    const MEMORY_MONITOR_MIN_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.minimum;
    const MEMORY_MONITOR_MAX_ROWS = ProjectSchema.properties.settings.properties.memoryMonitorMaxRows.maximum;

    export default {
        name: "EnvironmentSettingEnvInterface",
        mixins: [MixinSettingsPage],

        methods: {
            syncMemMonitorRowsLimit() {
                let value = parseInt(this.$refs.memMonRowsInput.value);

                let validValue = Math.min(MEMORY_MONITOR_MAX_ROWS,
                    Math.max(MEMORY_MONITOR_MIN_ROWS, value)
                );

                this.interfaceIsFreezed = true;

                let dialog = UIkit.modal.dialog(`
                    <div class="uk-flex uk-flex-center uk-flex-middle" style="height: 50vh">
                        <h2>
                            ${this.$t("guiCommon.wait")}
                        </h2>
                    </div>

                `, {
                    stack: true,
                    container: true,

                    escClose: false,
                    bgClose: false,

                });


                // it needs to be wrapped this way to work
                setTimeout(_ => {
                    new Promise(_ => {
                        this.projectSettings.memoryMonitorMaxRows = validValue;
                        dialog.hide();
                    });
                }, 500);
            }
        }
    }
</script>
