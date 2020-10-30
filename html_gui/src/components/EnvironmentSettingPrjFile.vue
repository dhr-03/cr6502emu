<template>
    <div class="uk-form-stacked">

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

                dom-id="confirmDeleteProject"

                ref="modal"
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

    </div>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import Modal from "./Modal";

    export default {
        name: "EnvironmentSettingPrjFile",
        components: {Modal},
        niceName: "File",
        mixins: [MixinSettingsPage],

        methods: {
            async downloadProject(e) {
                let projectData = await this.$store.dispatch("env/saveProjectState");

                if (e.shiftKey) {
                    // set timestamps to unix 0 + 1 day
                    projectData.meta.created = 24 * 60 * 60 * 1000;
                    projectData.meta.lastMod = 24 * 60 * 60 * 1000;
                }

                let encodedProjectData = btoa(
                    unescape(encodeURIComponent( //utf8 fix
                        JSON.stringify(projectData)
                    ))
                );

                let dwnNode = this.$refs.downloadTrick;

                let dwnFileName = "";
                for (let i = 0; i < projectData.meta.name.length; i++) {
                    let normalizedChar = projectData.meta.name[i]
                        .toLowerCase()
                        .replace(" ", "_")
                        .replace(/[^a-z-_]/, "");

                    dwnFileName += normalizedChar;
                }

                dwnNode.href = `data:application/octet-stream;base64,${encodedProjectData}`;
                dwnNode.download = `${dwnFileName}.cremu`;

                dwnNode.click();

                dwnNode.href = "";
                dwnNode.download = "";

            },

            cancelDelete() {
                this.$refs.modal.hideModal();
            },

            performDelete() {
                let pidToDelete = this.projectMeta.pid;

                this.$router.push({
                    name: "Home"
                }, _ => {
                    this.$store.dispatch("prj/completelyDeleteProjectById", pidToDelete);
                });
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/modifierStyles";

    .crl-button {
        width: 15em; //same width for all
    }
</style>
