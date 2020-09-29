<template>
    <form class="uk-form-stacked">

        <div class="uk-margin">
            <label class="uk-form-label">Project Name</label>
            <input
                v-model="projectMeta.name"

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

    </form>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";

    export default {
        name: "EnvironmentSettingPrjMeta",

        niceName: "Project about",

        mixins: [MixinSettingsPage],

        computed: {
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
