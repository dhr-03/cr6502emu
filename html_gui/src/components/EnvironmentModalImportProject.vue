<template>
    <Modal
        :show-footer="false"

        dom-id="importProjectModal"
        close-button-type="default"

        content-class="crl-dark-modal"
        class="crl-project-import"
    >

        <template v-slot:toggle>
                    <span class="crl-project-import-button">
                        <font-awesome-icon icon="upload"/>
                        Import Project
                    </span>
        </template>


        <template v-slot:header>
            <h3 class="uk-light">Upload project</h3>
        </template>

        <template v-slot:body>
            <Alert
                v-if="errorMessage != null"

                type="err"
                title="Failed to import file"
            >
                {{ errorMessage }}
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

                <button class="uk-button uk-button-default" type="button" tabindex="-1">Select File</button>
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

    import UIkit from "uikit";

    export default {
        name: "EnvironmentModalImportProject",
        components: {Modal, Alert},

        data() {
            return {
                errorMessage: null
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
                                    this.errorMessage = null;

                                    this.closeModal();

                                    this.$router.push({
                                        name: "Project",

                                        params: {
                                            pid: prj.meta.pid,
                                        }
                                    })

                                } else {
                                    switch (value.err) {
                                        case "validate":
                                            this.errorMessage = "Invalid file data.";
                                            break;

                                        case "exists":
                                            this.errorMessage = `A project with the id ${prj.meta.pid} already exists.`
                                            break;

                                        default:
                                            this.errorMessage = "Unknown error."
                                            break;
                                    }
                                }
                            });

                        } catch (e) {
                            this.errorMessage = "Invalid file structure.";
                        }
                    };

                    reader.readAsText(file);
                }
            },

            closeModal() {
                UIkit.modal("#importProjectModal").hide();
            }

        }
    }
</script>

<style lang="less">
    @import "../assets/less/darkModal";
</style>
