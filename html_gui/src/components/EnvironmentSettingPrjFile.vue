<template>
    <div class="uk-form-stacked">

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjFile.downloadPrj'"
            />
            <button
                @click="downloadProject"

                class="crl-button cr-info uk-button"
            >
                <font-awesome-icon icon="download"/>
                <span
                    class="cr-mg-t"
                    v-t="'environment.settings.EnvironmentSettingPrjFile.downloadPrj'"
                />
            </button>
        </div>

        <hr>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjFile.downloadPrj'"
            />

            <Modal
                :allow-stack="true"

                dom-id="confirmDeleteProject"

                ref="modal"
            >

                <template v-slot:toggle>
                    <button class="crl-button cr-err uk-button">
                        <font-awesome-icon icon="trash-alt"/>
                        <span
                            class="cr-mg-t"
                            v-t="'environment.settings.EnvironmentSettingPrjFile.deletePrj'"
                        />
                    </button>
                </template>


                <template v-slot:header>
                    <span
                        class="uk-modal-title uk-text-large"
                        v-t="'environment.settings.EnvironmentSettingPrjFile.deletePrj'"
                    />
                </template>

                <template v-slot:body>
                    <p>
                        <span v-t="'environment.settings.EnvironmentSettingPrjFile.deletePrompt.nameTitle'"/>
                        "<strong>{{ projectMeta.name }}</strong>"
                    </p>

                    <p>
                        <span v-t="'environment.settings.EnvironmentSettingPrjFile.deletePrompt.idTitle'"/>
                        "<strong>{{ projectMeta.pid }}</strong>" ?
                    </p>
                </template>

                <template v-slot:footer>
                    <div class="uk-button-group">
                        <button
                            @click="cancelDelete"

                            class="cr-info uk-button uk-margin-right"
                            v-t="'guiCommon.button.cancel'"
                        />

                        <button
                            @click="performDelete"

                            class="cr-err uk-button"
                            v-t="'guiCommon.button.delete'"
                        />
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
                        .replace(/\W/, "");

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
