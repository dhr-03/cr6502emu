<template>
    <div
            class="cr-alert"
            :class="typeClass"
    >
        <div class="cr-alert-icon">
            <font-awesome-icon :icon="iconName"/>
        </div>

        <div class="cr-alert-content">
            <slot name="title">
                <b v-if="title !== ''">
                    {{ title }}:
                </b>
            </slot>

            <slot>
                {{ text }}
            </slot>
        </div>

    </div>
</template>

<script>
    import MixinStatusElement from "./MixinStatusElement"

    export default {
        name: "Alert",
        mixins: [MixinStatusElement],

        props: {
            title: {
                type: String,
                default: _ => "",
            },

            text: {
                type: String,
                default: _ => "",
            }
        },

    }
</script>

<style lang="less" scoped>
    @import "../assets/less/modifierStyles";

    .cr-alert {
        display: table;

        padding: 0.5em;
        margin-bottom: 1em;

        width: 100%;
        min-height: 3em;

        .cr-alert-icon {
            display: table-cell;

            vertical-align: middle;

            padding-right: 1em;
            padding-left: 1em;
            border-right: 1px solid;
            width: 1.5em !important;

            font-size: 1.1em;

        }

        .cr-alert-content {
            display: table-cell;

            vertical-align: middle;
            padding-left: 1em;
        }
    }
</style>
