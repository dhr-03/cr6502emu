<template>
    <div
        class="crg-container cr-initialize"
        v-if="isInitializing"
    >
        <font-awesome-icon class="uk-margin-small" icon="cog" spin size="4x"/>
        <span>Initializing</span>
    </div>

    <div
        class="crg-container cr-initialize"
        v-else-if="!successfulInitialize"
    >
        <font-awesome-icon class="uk-margin-small cr-red" icon="times-circle" size="4x"/>
        <span>Failed to initialize Environment</span>

        <br>
        <code style="background: #fff; color: red; padding: 1em">{{ initErrorMessage }}</code>
    </div>

    <div v-else>
        <EnvironmentActionbar/>

        <div class="cr-environment uk-container uk-container-xlarge">

            <Alert
                v-if="targetProgramRomIndex == null"

                type="warn"
            >
                No <strong>Rom</strong> Selected
            </Alert>

            <div class="uk-grid uk-grid-small">
                <div class="uk-width-expand">

                    <EnvironmentEditor
                        :initial-code="editorInitialCode"

                        :key-up-callback="scheduleCurrentProjectSave"
                    />

                </div>

                <div class="uk-width-auto">
                    <EnvironmentWidgetsHolder class="cr-devholder-main"/>
                </div>
            </div>

            <EnvironmentWidgetsHolder class="cr-devholder-pool">

                <EnvironmentWidget
                    v-for="device in deviceList"
                    :key="device.uid"

                    :device="device"

                />

            </EnvironmentWidgetsHolder>
        </div>

        <EnvironmentLogbar/>
    </div>
</template>

<script>
    import EnvironmentEditor from "../components/EnvironmentEditor"
    import EnvironmentWidgetsHolder from "../components/EnvironmentWidgetsHolder"
    import EnvironmentWidget from "../components/EnvironmentWidget"
    import EnvironmentActionbar from "../components/EnvironmentActionbar"
    import EnvironmentLogbar from "../components/EnvironmentLogbar"
    import {mapGetters, mapActions} from "vuex"
    import Alert from "../components/Alert";

    export default {
        name: "Environment",

        props: {
            pid: {
                type: String,
                required: false
            }
        },

        components: {
            Alert,
            EnvironmentLogbar,
            EnvironmentActionbar, EnvironmentWidget, EnvironmentWidgetsHolder, EnvironmentEditor
        },

        computed: mapGetters("env", [
            "isInitializing",
            "successfulInitialize",
            "isExecuting",
            "deviceList",
            "editorInitialCode",
            "initErrorMessage",
            "targetProgramRomIndex",
        ]),

        methods: {
            ...mapGetters("env", [
                "currentProjectId",
            ]),

            ...mapActions("prj", [
                "loadProjectFromId",
                "saveCurrentProject",
                "scheduleCurrentProjectSave",
            ]),
        },

        async beforeRouteEnter(to, from, next) {
            next(vm => {
                vm.loadProjectFromId(to.params.pid)
            });
        },

        async beforeRouteUpdate(to) {
            await this.loadProjectFromId(to.params.pid);
        },

        async beforeRouteLeave(to, from, next) {
            if (this.currentProjectId()) {
                await this.saveCurrentProject();
            }

            next();
        },
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/namedColors";

    //TODO: temp
    .cr-initialize {
        display: flex;
        height: 70vh;

        justify-content: center;
        align-items: center;
        align-self: center;

        flex-direction: column;

        margin-top: 10vh;

    }

    .cr-environment {
        min-height: 115vh; //needed to scroll and hide the navbar
    }

    .cr-devholder-main {
        height: 100%;
    }

    .cr-devholder-pool {
        margin-top: 3em;
    }

</style>
