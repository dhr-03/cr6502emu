<template>
    <div class="uk-form-stacked">

        <div class="uk-margin">
            <label class="uk-form-label">Project Name</label>
            <input
                v-model="prjName"

                required

                class="uk-input"
                type="text"
                placeholder="Project Name"
            >
        </div>

        <div class="uk-margin">
            <label class="uk-form-label">Project Id</label>
            <input
                :value="projectMeta.pid"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

        <hr>

        <div class="uk-margin">
            <label class="uk-form-label">Last project activity</label>
            <input
                :value="prjNiceLastMod"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

        <div class="uk-margin">
            <label class="uk-form-label">Project creation date</label>
            <input
                :value="prjNiceCreationDate"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

    </div>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";
    import {ProjectSchema} from "../assets/schema/project";

    const DefaultPrjName = ProjectSchema.properties.meta.properties.name.default;

    export default {
        name: "EnvironmentSettingPrjMeta",

        niceName: "Project about",

        mixins: [MixinSettingsPage],

        computed: {
            prjName: {
                get() {
                    return this.projectMeta.name;
                },

                set(value) {
                    this.projectMeta.name = value || DefaultPrjName;
                }
            },

            prjNiceLastMod() {
                let date = new Date(this.projectMeta.lastMod);

                return date.toLocaleString();
            },

            prjNiceCreationDate() {
                let date = new Date(this.projectMeta.created);

                return date.toLocaleString();
            },

        }
    }
</script>

<style lang="less" scoped>

</style>
