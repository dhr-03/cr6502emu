<template>
    <div class="crl-widget" ref="canvasContainer"></div>
</template>

<script>
    import MixinEnvironmentWidget from "./MixinEnvironmentWidget";

    export default {
        name: "EnvironmentWidgetPixelScreen",
        mixins: [MixinEnvironmentWidget],

        mounted() {
            this.$refs.canvasContainer.appendChild(this.widget.displayData.canvas);
        }
    }
</script>

<style lang="less">
    .crl-widget canvas {
        padding: 0.5em;
        width: 21em;
        height: 12.5em;
    }
</style>
