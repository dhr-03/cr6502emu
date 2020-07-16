<template>
    <div
            class="cr-dev-holder uk-flex uk-flex-middle"
            uk-sortable="group: widgets; handle: .cr-widget-handle-container"
    >
        <div class="cr-dev-holder-empty">
        <span>
            <font-awesome-icon icon="plus"/>
        </span>
        </div>

        <slot></slot>
    </div>
</template>

<script>
    export default {
        name: "EnvironmentWidgetsHolder"
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-dev-holder {
        background: @oc-gray-8;

        min-width: 31em;
        min-height: 16em;

        border-radius: 10pt;

        padding-right: 0.5em;
    }

    .cr-dev-holder-empty {
        display: flex;

        width: 100%;

        height: 100%;
        justify-content: center;
        align-items: center;

        opacity: 15%;
        font-size: 300%;

        &:not(:only-child) {
            display: none;
        }
    }
</style>
