<template>
    <div
            class="cr-container cr-initialize"
            v-if="isInitializing"
    >
        <font-awesome-icon class="uk-margin-small" icon="cog" spin size="4x"/>
        <span>Initializing</span>
    </div>

    <div
            class="cr-container cr-initialize"
            v-else-if="!successfulInitialize"
    >
        <font-awesome-icon class="uk-margin-small cr-red" icon="times-circle" size="4x"/>
        <span>Failed to initialize Environment</span>
    </div>

    <div v-else>
        <EnvironmentActionbar
                @env_build="buildToRom"
                @env_reset="resetSystem"

                @env_toggle-run="toggleRun"
                @env_toggle-debug="toggleDebug"

                @env_step-short="cpuShortStep"
        />

        <div class="cr-environment uk-container uk-container-xlarge">
            <div class="uk-grid uk-grid-small">
                <div class="uk-width-expand">

                    <EnvironmentEditor/>

                </div>

                <div class="uk-width-auto">
                    <EnvironmentWidgetsHolder class="cr-devholder-main"/>
                </div>
            </div>

            <EnvironmentWidgetsHolder class="cr-devholder-pool"/>
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
            "isExecuting"
        ]),

        methods: mapActions("env", [
            "buildToRom",
            "resetSystem",
            "toggleRun",
            "toggleDebug",
            "cpuShortStep"
        ]),

        created() {
            this.$store.dispatch("env/setup",
                _ => this.$store.dispatch("env/initialize")
            );
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


    /* override default uikit styles */
    .uk-notification {
        width: fit-content;
    }

    .uk-notification-message {
        background: #EEE;
    }

    .uk-notification a {
        display: none !important;
    }

    /* custom animation */
    @keyframes rotate-opacity {
        0% {
            opacity: 0;
        }

        100% {
            transform: rotate(360deg);
            opacity: 1;
        }
    }

    .cr-animation-rotate-opacity {
        animation: rotate-opacity 0.5s ease;
    }
</style>
