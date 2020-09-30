<template>
    <Modal
        :allow-stack="true"

        content-class="crl-dark-modal"
        dom-id="swapDevicesPrompt"
    >

        <template v-slot:toggle>
            <button class="cr-info uk-button">
                <font-awesome-icon icon="random"/>
                Swap devices
            </button>
        </template>


        <template v-slot:header>
            <h3 class="uk-modal-title uk-light">Swap devices</h3>
        </template>

        <template v-slot:body>
            <div class="uk-form-stacked uk-light">
                <Alert
                    v-if="failedToSwap"

                    type="err"
                >
                    Failed to swap devices.
                </Alert>


                <div class="uk-margin">
                    <label class="uk-form-label">Device A</label>
                    <select
                        v-model="deviceA"

                        class="uk-select"
                    >
                        <option value="null" class="uk-hidden">Select one</option>

                        <option
                            v-for="(devNames, index) in devicesReprString"
                            :key="index"

                            :value="index"
                        >
                            {{ devNames }}
                        </option>
                    </select>
                </div>

                <div class="uk-margin">
                    <label class="uk-form-label">Device B</label>
                    <select
                        v-model="deviceB"

                        class="uk-select"
                    >
                        <option value="null" class="uk-hidden">Select one</option>

                        <option
                            v-for="(devNames, index) in devicesReprString"
                            :key="index"

                            :value="index"
                        >
                            {{ devNames }}
                        </option>
                    </select>
                </div>


            </div>
        </template>

        <template v-slot:footer>
            <button
                @click="swapDevices"

                :disabled="!allowSubmit"

                class="uk-button uk-button-primary"
            >
                Swap devices
            </button>
        </template>
    </Modal>
</template>

<script>
    import Modal from "./Modal";
    import {mapActions, mapGetters} from "vuex";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";
    import Alert from "./Alert";

    import UIkit from "uikit";

    export default {
        name: "EnvironmentModalSwapDevices",
        mixins: [MixinPreferredNumericBase],
        components: {Alert, Modal},

        data() {
            return {
                deviceA: null,
                deviceB: null,

                failedToSwap: false,
            }
        },

        computed: {
            ...mapGetters("env", [
                "deviceListWithoutCpu"
            ]),

            devicesReprString() {
                return this.deviceListWithoutCpu.map(dev => dev.getRepresentationString());
            },

            allowSubmit() {
                return (this.deviceA != null && this.deviceB != null) &&
                    this.deviceA !== this.deviceB;
            },
        },

        methods: {
            ...mapActions("env", [
                "swapDevicesByIndex"
            ]),

            swapDevices() {
                this.swapDevicesByIndex([this.deviceA + 1, this.deviceB + 1]).then(success => {
                    this.failedToSwap = !success;

                    //TODO: tmp
                    if(success) UIkit.modal("#swapDevicesPrompt").hide();
                })
            }
        },
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/darkModal";
</style>
