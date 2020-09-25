<template>
    <div>
        <br>
        <br>

        <div class="js-upload uk-flex uk-flex-center" uk-form-custom>
            <input
                @change="onFileInput"

                type="file"
                accept=".cremu"
            >

            <button class="uk-button uk-button-default" type="button" tabindex="-1">Select File</button>
        </div>

        <br>
        <br>
    </div>
</template>

<script>
    export default {
        name: "EnvironmentImportProject",

        methods: {
            onFileInput(e) {
            }
        }
    }
</script>

<style lang="less" scoped>

</style>
