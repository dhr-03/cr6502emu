<template>
    <div>
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
    </div>
</template>

<script>
    import {mapActions} from "vuex";
    import Alert from "./Alert";

    export default {
        name: "EnvironmentImportProject",
        components: {Alert},

        props: {
            callback: {
                type: Function,
                required: false
            }
        },

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

                                    if (this.callback != null) {
                                        this.callback();
                                    }

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
        }
    }
</script>

<style lang="less" scoped>

</style>
