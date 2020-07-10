<template>
    <li class="cr-separator">
        <i class="cr-separator-i"></i>
    </li>
</template>

<script>
    export default {
        name: "EnvironmentActionbarSeparator",
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-separator {
        padding: 1em 0.3em;

        .cr-separator-i {
            border-right: 1px solid @oc-gray-5;
            height: 150%;
            padding: 0;
        }
    }
</style>
