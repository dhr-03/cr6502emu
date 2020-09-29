<template>
    <div class="cr-widget uk-light">
        <div class="cr-widget-header uk-grid">

            <span
                class="cr-widget-title uk-width-expand"
                :title="title"
            >
                {{ title }}
            </span>

            <div class="cr-widget-handle-container uk-width-auto">
                <font-awesome-icon icon="bars"/>
            </div>
        </div>

        <div class="cr-widget-square">
            <div class="cr-widget-container">
                <component
                    :is="bodyElement"
                    :device="device"
                />
            </div>
        </div>
    </div>

</template>

<script>
    import {DeviceRepresentation} from "../assets/js/deviceRepresentation/deviceRepresentation";

    export default {
        name: "EnvironmentWidget",

        props: {
            device: {
                type: DeviceRepresentation,
                required: true,
            },
        },

        data() {
            return {
                bodyElement: this.device.widgetComponent
            };
        },

        computed: {
            title() {
                return this.device.widget.config.title || this.device.niceName;
            },
        }
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-widget {
        display: inline-block;
        background: @oc-gray-6;

        width: 15em;

        margin: 0.5em 0 0.5em 0.5em;

        overflow: hidden;
        border-radius: 4pt;
    }

    .cr-widget-header {
        padding: 0.5em 0.5em 0.2em;
    }

    .cr-widget-title {
        white-space: nowrap;

        overflow: hidden;
        text-overflow: ellipsis;

        font-weight: bold;
    }

    .cr-widget-handle-container {
        padding-left: 0.5em;
        z-index: 1;

        cursor: grab;
    }

    .cr-widget-square {
        width: 100%;
        overflow-y: auto;
        padding-bottom: 85%;
        position: relative;

        border-top: 1px solid #333;
    }

    .cr-widget-container {
        position: absolute;

        background: @oc-gray-4;

        min-width: 100%;
        min-height: 100%;

        border-radius: 0 0 4pt 4pt;
    }
</style>
