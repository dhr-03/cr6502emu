<template>
    <form class="uk-form-stacked">

        <div class="uk-margin">
            <label class="uk-form-label">Download Project</label>
            <button
                @click="downloadProject"

                class="crl-button cr-info uk-button"
            >
                <font-awesome-icon icon="download"/>
                Download project
            </button>
        </div>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Delete Project</label>

            <Modal
                :allow-stack="true"

                dom-id="confirm-delete"
                content-class="crl-dark-modal"
            >

                <template v-slot:toggle>
                    <button class="crl-button cr-err uk-button">
                        <font-awesome-icon icon="trash-alt"/>
                        Delete Project
                    </button>
                </template>


                <template v-slot:header>
                    <span class="uk-modal-title uk-text-large">Delete Project</span>
                </template>

                <template v-slot:body>
                    <p>
                        Delete project: "<strong>{{ projectMeta.name }}</strong>"
                        id: "<strong>{{ projectMeta.pid }}</strong>" ?
                    </p>
                </template>

                <template v-slot:footer>
                    <div class="uk-button-group">
                        <button
                            @click="cancelDelete"

                            class="cr-info uk-button uk-margin-right"
                        >
                            Cancel
                        </button>

                        <button
                            @click="performDelete"

                            class="cr-err uk-button"
                        >
                            Delete
                        </button>
                    </div>
                </template>

            </Modal>
        </div>

        <a style="display: none" ref="downloadTrick"></a>

    </form>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import Modal from "./Modal";

    import UIkit from "uikit";

    export default {
        name: "EnvironmentSettingPrjFile",
        components: {Modal},
        niceName: "File",
        mixins: [MixinSettingsPage],

        methods: {
            async downloadProject() {
                let projectData = JSON.stringify(
                     await this.$store.dispatch("env/saveProjectState")
                );

                let dwnNode = this.$refs.downloadTrick;

                dwnNode.href = `data:application/octet-stream;base64,${btoa(projectData)}`;
                dwnNode.download = `${this.projectMeta.name}.cremu`;

                dwnNode.click();

                dwnNode.href = "";
                dwnNode.download = "";

            },

            cancelDelete() {
                UIkit.modal("#confirm-delete").hide();
            },

            performDelete() {
                let pidToDelete = this.projectMeta.pid;

                this.$router.push({
                    name: "Home"
                }, _ => {
                    //TODO: tmp
                    UIkit.modal("#confirm-delete").hide();
                    UIkit.modal("#settings-menu").hide();

                    this.$store.dispatch("prj/completelyDeleteProjectById", pidToDelete);
                });
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/modifierStyles";
    @import "../assets/less/darkModal";

    .crl-button {
        width: 15em; //same width for all
    }
</style>
