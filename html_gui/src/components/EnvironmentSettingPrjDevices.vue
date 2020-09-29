<template>
    <form class="uk-form-stacked">

        <table class="uk-table uk-table-divider uk-table-hover uk-light">
            <thead>
            <tr>
                <th>UID</th>
                <th>Type</th>

                <th>Start</th>
                <th>End</th>
                <th>Size</th>

                <th>Actions</th>
            </tr>
            </thead>

            <tbody>

            <tr
                v-for="(device, index) of deviceListWithoutCpu"
                :key="index"
            >

                <td>{{device.uid}}</td>
                <td>{{device.type}}</td>

                <td>
                    <EnvironmentNumberContainer
                        :base="projectSettings.preferredNumericBase"

                        :value="device.start"
                        :length-in-bytes="2"
                    />
                </td>

                <td>
                    <EnvironmentNumberContainer
                        :base="projectSettings.preferredNumericBase"

                        :value="device.end"
                        :length-in-bytes="2"
                    />
                </td>

                <td>
                    <EnvironmentNumberContainer
                        :base="projectSettings.preferredNumericBase"

                        :value="device.size"
                        :length-in-bytes="2"
                    />
                </td>

                <td>

                </td>
            </tr>

            <tr
                v-if="!deviceListWithoutCpu.length"

                class="uk-text-center"
            >
                <td colspan="6">No devices found.</td>
            </tr>

            </tbody>
        </table>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Project creation date</label>
            <input
                :value="prjNiceCreationDate"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

    </form>
</template>

<script>
    import {mapGetters} from "vuex";

    import MixinSettingsPage from "./MixinSettingsPage";
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";


    export default {
        name: "EnvironmentSettingPrjDevices",
        components: {EnvironmentNumberContainer},
        niceName: "Devices",

        mixins: [MixinSettingsPage],

        computed: {
            ...mapGetters("env", [
                "deviceListWithoutCpu"
            ])
        }
    }
</script>

<style lang="less" scoped>

</style>
