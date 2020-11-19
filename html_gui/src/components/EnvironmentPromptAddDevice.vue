<template>
    <Modal
        :allow-stack="true"

        dom-id="addDevicePrompt"

        ref="modal"
    >
        <template v-slot:toggle>
            <button class="cr-info uk-button">
                <font-awesome-icon icon="plus"/>
                <span
                    class="cr-mg-t"
                    v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.button.toggle'"
                />
            </button>
        </template>


        <template v-slot:header>
            <h3
                class="uk-modal-title uk-light"
                v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.title'"
            />
        </template>

        <template v-slot:body>
            <Alert
                v-if="failedToAdd"

                type="err"
            >
                <span
                    v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.failedToAdd'"
                />
            </Alert>

            <div class="uk-form-stacked uk-light">

                <div class="uk-grid">

                    <div class="uk-width-2-3">
                        <label
                            class="uk-form-label"
                            v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.dev.type'"
                        />
                        <select
                            v-model="selectedDeviceId"

                            class="uk-select"
                        >
                            <option
                                v-for="(repr, index) of devicesToAdd"
                                :key="index"

                                :value="repr.type"
                            >
                                {{ repr.niceName }}
                            </option>
                        </select>
                    </div>

                    <div class="uk-width-1-3">
                        <label
                            class="uk-form-label"
                            v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.numBase'"
                        />
                        <select
                            v-model.number="inputNumericBase"

                            class="uk-select"
                        >
                            <option value="10" v-t="'guiCommon.numericBase.dec'"></option>
                            <option value="16" v-t="'guiCommon.numericBase.hex'"></option>
                        </select>
                    </div>
                </div>

                <hr>

                <div class="uk-grid uk-margin-top">
                    <div class="uk-width-1-2">
                        <label
                            class="uk-form-label"
                            v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.dev.addr'"
                        />
                        <NumericInput
                            :numeric-base="inputNumericBase"

                            :max-value="addrInputMaxValue"

                            @value-changed="setAddrValue"
                        />
                    </div>

                    <div class="uk-width-1-2">
                        <label
                            class="uk-form-label"
                            v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.dev.size'"
                        />
                        <input
                            v-if="sizeInputIsDisabled"

                            disabled
                            placeholder="Fixed"

                            type="text"
                            class="uk-input"
                        >

                        <NumericInput
                            v-if="!sizeInputIsDisabled"

                            :numeric-base="inputNumericBase"

                            :min-value="1"
                            :initial-value="1"

                            @value-changed="setSizeValue"
                        />
                    </div>
                </div>
            </div>
        </template>

        <template v-slot:footer>
            <button
                @click="addDevice"

                :disabled="!allowSubmit"

                class="uk-button uk-button-primary"
                v-t="'environment.settings.EnvironmentSettingPrjDevices.addPrompt.button.submit'"
            />
        </template>

    </Modal>

</template>

<script>
    import {mapActions} from "vuex";

    const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

    import Tools from "../assets/js/tools";

    import Modal from "./Modal";
    import NumericInput from "./NumericInput";

    import {RepresentedPixelScreen} from "../assets/js/deviceRepresentation/representedPixelScreen";
    import {RepresentedAsciiBuffer} from "../assets/js/deviceRepresentation/representedAsciiBuffer";
    import {RepresentedRom} from "../assets/js/deviceRepresentation/representedRom";
    import {RepresentedRam} from "../assets/js/deviceRepresentation/representedRam";

    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";

    import Alert from "./Alert";

    export default {
        name: "EnvironmentPromptAddDevice",
        mixins: [MixinPreferredNumericBase],
        components: {Alert, NumericInput, Modal},

        data() {
            return {
                selectedDeviceId: RepresentedRam.type,
                inputNumericBase: null,

                deviceSize: "1000",

                addrValue: null,
                sizeValue: null,

                failedToAdd: false,
            }
        },

        computed: {
            devicesToAdd() {
                return [
                    RepresentedRom,
                    RepresentedRam,

                    RepresentedPixelScreen,
                    RepresentedAsciiBuffer,
                ];
            },

            selectedDevice() {
                return this.devicesToAdd.find(dev => dev.type === this.selectedDeviceId);
            },

            sizeInputIsDisabled() {
                return this.selectedDevice.hasFixedSize;
            },

            allowSubmit() {
                let valuesAreValid = this.addrValue != null && (this.sizeInputIsDisabled || this.sizeValue != null);

                let rangeIsValid = (this.addrValue + (this.sizeValue || 0)) <= Tools.U16MaxValue;

                return valuesAreValid && rangeIsValid;
            },

            addrInputMaxValue() {
                return Tools.U16MaxValue - 1;
            }

        },

        methods: {
            ...mapActions("env", [
                "addDeviceWithWidget"
            ]),

            setAddrValue(value) {
                this.addrValue = value;
            },

            setSizeValue(value) {
                this.sizeValue = value;
            },

            addDevice() {
                this.addDeviceWithWidget({
                    type: this.selectedDeviceId,

                    start: this.addrValue,
                    size: this.sizeInputIsDisabled ? 0 : this.sizeValue || 0,
                }).then(success => {
                    this.failedToAdd = !success;

                    if (success) {
                        this.$refs.modal.hideModal();
                    }
                });
            },
        },

        created() {
            this.inputNumericBase = this.preferredNumericBase;
        }
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/modifierStyles";
</style>
