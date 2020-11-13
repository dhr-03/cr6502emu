<template>
    <div class="uk-form-stacked">

        <table class="uk-table uk-table-divider uk-table-hover uk-table-middle uk-light">
            <thead>
            <tr>
                <th v-t="'environment.settings.EnvironmentSettingPrjDevices.table.uid'"/>
                <th v-t="'environment.settings.EnvironmentSettingPrjDevices.table.type'"/>

                <th v-t="'environment.settings.EnvironmentSettingPrjDevices.table.start'"/>
                <th v-t="'environment.settings.EnvironmentSettingPrjDevices.table.end'"/>
                <th v-t="'environment.settings.EnvironmentSettingPrjDevices.table.size'"/>

                <th></th>

                <th
                    class="uk-table-shrink"
                    v-t="'environment.settings.EnvironmentSettingPrjDevices.table.actions'"
                />
            </tr>
            </thead>

            <tbody>

            <tr
                v-for="(device, index) of deviceListWithoutCpu"
                :key="index"
            >

                <td>{{ device.uid }}</td>
                <td>{{ device.constructor.niceName }}</td>

                <td>
                    <EnvironmentNumberContainer
                        :base="preferredNumericBase"

                        :value="device.start"
                        :length-in-bytes="2"
                    />
                </td>

                <td>
                    <EnvironmentNumberContainer
                        :base="preferredNumericBase"

                        :value="device.end"
                        :length-in-bytes="2"
                    />
                </td>

                <td>
                    <EnvironmentNumberContainer
                        :base="preferredNumericBase"

                        :value="device.size"
                        :length-in-bytes="2"
                    />
                </td>

                <td></td>

                <td class="uk-text-center">
                    <button
                        @click="removeDeviceById(device.uid)"

                        class="cr-err uk-button uk-padding-small"
                    >
                        <font-awesome-icon icon="trash-alt"/>
                    </button>
                </td>
            </tr>

            <tr
                v-if="!deviceListWithoutCpu.length"

                class="uk-text-center"
            >
                <td
                    colspan="7"
                    v-t="'environment.settings.EnvironmentSettingPrjDevices.table.emptyMessage'"
                />
            </tr>

            </tbody>
        </table>

        <div class="uk-flex uk-flex-around">
            <EnvironmentPromptAddDevice/>

            <EnvironmentPromptSwapDevices/>
        </div>

        <hr>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjDevices.buildRom.title'"
            />
            <select
                v-model="selectedRomId"

                class="uk-select"
            >
                <option class="uk-hidden" value="null">
                    {{
                        $t("environment.settings.EnvironmentSettingPrjDevices.buildRom."
                            + (romList.length ? "nonEmptyMessage" : "emptyMessage")
                        )
                    }}
                </option>

                <option
                    v-for="rom in romList"

                    :value="rom.uid"
                >
                    {{ rom.getRepresentationString(preferredNumericBase) }}
                </option>
            </select>
        </div>

    </div>
</template>

<script>
    import {mapActions, mapGetters, mapMutations} from "vuex";

    const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

    import MixinSettingsPage from "./MixinSettingsPage";
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";
    import EnvironmentPromptAddDevice from "./EnvironmentPromptAddDevice";
    import EnvironmentPromptSwapDevices from "./EnvironmentPromptSwapDevices";


    export default {
        name: "EnvironmentSettingPrjDevices",
        mixins: [MixinSettingsPage, MixinPreferredNumericBase],
        components: {EnvironmentPromptSwapDevices, EnvironmentPromptAddDevice, EnvironmentNumberContainer},

        computed: {
            ...mapGetters("env", [
                "deviceListWithoutCpu",
                "targetProgramRomId",
            ]),

            romList() {
                return this.deviceListWithoutCpu.filter(dev => dev.constructor.type === DeviceId.Rom);
            },

            selectedRomId: {
                get() {
                    return this.targetProgramRomId;
                },

                set(id) {
                    this.updateProgramRomId(id)
                }
            }
        },

        methods: {
            ...mapActions("env", [
                "removeDeviceById",
                "updateProgramRomId",
            ]),
        },
    }
</script>

<style lang="less">
    @import "../assets/less/modifierStyles";
</style>
