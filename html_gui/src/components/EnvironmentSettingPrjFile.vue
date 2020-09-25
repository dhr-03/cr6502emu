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
            <button class="crl-button cr-err uk-button">
                <font-awesome-icon icon="trash-alt"/>
                Delete Project
            </button>
        </div>

        <a style="display: none" ref="downloadTrick"></a>

    </form>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";

    export default {
        name: "EnvironmentSettingPrjFile",
        niceName: "File",
        mixins: [MixinSettingsPage],

        methods: {
            downloadProject() {
                let projectData = JSON.stringify(
                    this.$store.dispatch("env/saveProjectState")
                );

                let dwnNode = this.$refs.downloadTrick;

                dwnNode.href = `data:application/octet-stream;,${projectData}`;
                dwnNode.download = `${this.projectMeta.name}.cremu`;

                dwnNode.click();

                dwnNode.href = "";
                dwnNode.download = "";

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
