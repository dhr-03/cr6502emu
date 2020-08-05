<template>
    <div class="cr-widget-cpu">
        <EnvironmentWidgetCpuRegister
            name="A"
            :value="widget.displayData.a"
        />

        <EnvironmentWidgetCpuRegister
            name="X"
            :value="widget.displayData.x"
        />

        <EnvironmentWidgetCpuRegister
            name="Y"
            :value="widget.displayData.y"
        />


        <hr class="crl-separator">


        <EnvironmentWidgetCpuRegister
            name="PC"
            :value="widget.displayData.pc"

            :length-in-bytes="2"
        />
        <EnvironmentWidgetCpuRegister
            name="Flags"
            :value="widget.displayData.p"
        />


        <hr class="crl-separator">


        <EnvironmentWidgetCpuRegister
            name="Bus Addr"
            :value="widget.displayData.busAddr"

            :length-in-bytes="2"
        />

        <EnvironmentWidgetCpuRegister
            name="Bus Data"
            :value="widget.displayData.busData"
        />

    </div>
</template>

<script>
    import EnvironmentWidgetCpuRegister from "./EnvironmentWidgetCpuRegister";
    import MixinEnvironmentWidget from "./MixinEnvironmentWidget";

    export default {
        name: "EnvironmentWidgetCpu",
        mixins: [MixinEnvironmentWidget],

        components: {EnvironmentWidgetCpuRegister}
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .cr-widget-cpu {
        margin: 1em;

        background: #fff;
        color: black;
    }

    .crl-reg-name:after {
        float: right;
        content: "⟶";
    }

    .crl-separator {
        background: black;
        margin: 10px 5px 10px 5px;
    }
</style>
