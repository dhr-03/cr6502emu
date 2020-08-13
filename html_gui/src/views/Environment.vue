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
    </div>

    <div v-else>
        <EnvironmentActionbar/>

        <div class="cr-environment uk-container uk-container-xlarge">
            <div class="uk-grid uk-grid-small">
                <div class="uk-width-expand">

                    <EnvironmentEditor/>

                </div>

                <div class="uk-width-auto">
                    <EnvironmentWidgetsHolder class="cr-devholder-main"/>
                </div>
            </div>

            <EnvironmentWidgetsHolder class="cr-devholder-pool">

                <EnvironmentWidget
                    v-for="(device, index) in deviceList"
                    :key="index"

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

    export default {
        name: "Environment",
        components: {
            EnvironmentLogbar,
            EnvironmentActionbar, EnvironmentWidget, EnvironmentWidgetsHolder, EnvironmentEditor
        },

        computed: mapGetters("env", [
            "isInitializing",
            "successfulInitialize",
            "isExecuting",
            "deviceList"
        ]),

        methods: mapActions("env", [
            "setup",
            "initialize",
        ]),

        created() {
            this.setup(_ => this.initialize());
        }
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
