<template>
    <form class="uk-form-stacked">

        <table class="uk-table uk-table-divider uk-table-hover uk-table-middle uk-light">
            <thead>
            <tr>
                <th>UID</th>
                <th>Type</th>

                <th>Start</th>
                <th>End</th>
                <th>Size</th>

                <th></th>

                <th class="uk-table-shrink">Actions</th>
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
                <td colspan="7">No devices found.</td>
            </tr>

            </tbody>
        </table>

        <div class="uk-flex uk-flex-around">
            <EnvironmentModalAddDevice/>

            <EnvironmentModalSwapDevices/>
        </div>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Build Rom</label>
            <select
                v-model="selectedRomId"

                class="uk-select"
            >
                <option class="uk-hidden" value="null">
                    {{ romList.length ? "Select One" : "None found" }}
                </option>

                <option
                    v-for="rom in romList"

                    :value="rom.uid"
                >
                    {{ rom.getRepresentationString() }}
                </option>
            </select>
        </div>

    </form>
</template>

<script>
    import {mapActions, mapGetters, mapMutations} from "vuex";

    const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

    import MixinSettingsPage from "./MixinSettingsPage";
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";
    import EnvironmentModalAddDevice from "./EnvironmentModalAddDevice";
    import EnvironmentModalSwapDevices from "./EnvironmentModalSwapDevices";


    export default {
        name: "EnvironmentSettingPrjDevices",
        mixins: [MixinSettingsPage, MixinPreferredNumericBase],
        components: {EnvironmentModalSwapDevices, EnvironmentModalAddDevice, EnvironmentNumberContainer},

        niceName: "Devices",

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
                    this.setTargetProgramRomId(id)
                }
            }
        },

        methods: {
            ...mapActions("env", [
                "removeDeviceById",
            ]),

            ...mapMutations("env", [
                "setTargetProgramRomId",
            ]),
        },
    }
</script>

<style lang="less">
    @import "../assets/less/modifierStyles";
</style>
