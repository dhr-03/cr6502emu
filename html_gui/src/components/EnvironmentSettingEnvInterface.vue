<template>
    <form class="uk-form-stacked">

        <div class="uk-margin">
            <label class="uk-form-label">Preferred numeric base</label>

            <select
                v-model.number="projectSettings.preferredNumericBase"

                class="uk-select"
            >

                <option value="10">Decimal (10)</option>
                <option value="16">Hexadecimal (16)</option>

            </select>
        </div>

    </form>
</template>

<script>
    import MixinSettingsPage from "./MixinSettingsPage";

    export default {
        name: "EnvironmentSettingPrjMeta",

        niceName: "Interface",

        mixins: [MixinSettingsPage],

        computed: {
        }
    }
</script>
