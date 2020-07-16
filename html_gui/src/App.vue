<template>
    <div id="app" class="uk-light">
        <TheNavbar/>

        <router-view/>
    </div>
</template>

<script>
    import TheNavbar from "./components/TheNavbar"
    export default {
        components: {TheNavbar}
    }
</script>

<style lang="less">
    @import "../node_modules/open-color/open-color";
    @import "../node_modules/uikit/src/less/uikit";

    #app {
        background: @oc-gray-7;

        min-height: 100%;
    }

    ::-webkit-scrollbar {
        width: 0.5em;
        height: 0.5em;
    }

    ::-webkit-scrollbar-corner {
        background: #444;
    }

    ::-webkit-scrollbar-track {
        background: #444
    }


    ::-webkit-scrollbar-thumb {
        background: #AAA;
    }


    ::-webkit-scrollbar-thumb:hover {
        background: #CCC;
    }

    .cr-container {
        .uk-container();

        background: darken(@oc-gray-7, 6%);
        border-radius: 2pt;

        padding: 1em;
    }

    .cr-mg-top {
        margin-top: 2em;
    }
</style>
