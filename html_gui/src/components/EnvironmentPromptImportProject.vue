<template>
    <Modal
        :show-footer="false"

        dom-id="importProjectPrompt"

        class="crl-project-import"

        ref="modal"
    >

        <template v-slot:toggle>
                    <span class="crl-project-import-button">
                        <font-awesome-icon icon="upload"/>
                        <span class="cr-mg-t" v-t="'projectChooser.button.import'"></span>
                    </span>
        </template>


        <template v-slot:header>
            <h3 class="uk-light" v-t="'projectChooser.importPrompt.title'"/>
        </template>

        <template v-slot:body>
            <Alert
                v-if="errorKey"

                type="err"
            >
                <template v-slot:title>
                    <span v-t="'projectChooser.importPrompt.error.title'"></span>
                </template>

                <template v-slot>
                    {{ $t(errorKey, errorData) }}
                </template>
            </Alert>
            <br>
            <br>

            <div class="js-upload uk-flex uk-flex-center" uk-form-custom>
                <input
                    @input="onFileUploaded"
                    ref="fileInput"

                    type="file"
                    accept=".cremu"
                >

                <button
                    class="uk-button uk-button-default"
                    v-t="'projectChooser.importPrompt.selectButtonText'"
                />
            </div>

            <br>
            <br>
        </template>

    </Modal>
</template>

<script>
    import {mapActions} from "vuex";
    import Alert from "./Alert";
    import Modal from "./Modal";

    export default {
        name: "EnvironmentPromptImportProject",
        components: {Modal, Alert},

        data() {
            return {
                errorKey: null,
                errorData: null,
            };
        },

        methods: {
            ...mapActions("prj", [
                "tryToImportProject"
            ]),

            onFileUploaded() {
                let file = this.$refs.fileInput.files[0];

                if (file != null) {
                    let reader = new FileReader();
                    reader.onload = e => {
                        try {
                            let prj = JSON.parse(e.target.result);

                            this.tryToImportProject(prj).then(value => {
                                if (value.ok) {
                                    this.errorKey = null;
                                    this.errorData = null;

                                    this.$refs.modal.hideModal();

                                    this.$router.push({
                                        name: "Project",

                                        params: {
                                            pid: prj.meta.pid,
                                        }
                                    })

                                } else {
                                    switch (value.err) {
                                        case "validate":
                                            this.errorKey = "projectChooser.importPrompt.error.invalidData";
                                            break;

                                        case "exists":
                                            this.errorKey = "projectChooser.importPrompt.error.unavailableId";
                                            this.errorData = {
                                                pid: prj.meta.pid
                                            };
                                            break;

                                        default:
                                            this.errorKey = "projectChooser.importPrompt.error.unknown";
                                            break;
                                    }
                                }
                            });

                        } catch (e) {
                            this.errorKey = "projectChooser.importPrompt.error.invalidFile";
                        }
                    };

                    reader.readAsText(file);
                }
            },
        }
    }
</script>

