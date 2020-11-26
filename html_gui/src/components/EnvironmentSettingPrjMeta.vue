<template>
    <div class="uk-form-stacked">

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjMeta.prjName'"
            />
            <input
                v-model="prjName"

                required

                class="uk-input"
                type="text"
                :placeholder="$t('environment.settings.EnvironmentSettingPrjMeta.prjName')"

                @focusout="checkNameNotEmpty"
            >
        </div>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjMeta.prjId'"
            />
            <input
                :value="projectMeta.pid"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

        <hr>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjMeta.dateLastMod'"
            />
            <input
                :value="prjNiceLastMod"

                class="uk-input"
                type="text"
                disabled
            >
        </div>

        <div class="uk-margin">
            <label
                class="uk-form-label"
                v-t="'environment.settings.EnvironmentSettingPrjMeta.dateCreated'"
            />
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
        mixins: [MixinSettingsPage],

        computed: {
            prjName: {
                get() {
                    return this.projectMeta.name;
                },

                set(value) {
                    this.projectMeta.name = value;
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

        },

        methods: {
            checkNameNotEmpty() {
                if (this.prjName.length < 3) {
                    this.prjName = DefaultPrjName;
                }
            }
        }
    }
</script>

<style lang="less" scoped>

</style>
