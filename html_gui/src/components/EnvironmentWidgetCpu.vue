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
            name="Stack Ptr"
            :value="widget.displayData.s"

            :length-in-bytes="1"
        />

        <EnvironmentWidgetCpuRegister
            name="Flags"
            value=""
        >
            <EnvironmentWidgetCpuFlags
                :value="widget.displayData.p"
            />
        </EnvironmentWidgetCpuRegister>


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
    import EnvironmentWidgetCpuFlags from "./EnvironmentWidgetCpuFlags";

    export default {
        name: "EnvironmentWidgetCpu",
        mixins: [MixinEnvironmentWidget],

        components: {EnvironmentWidgetCpuFlags, EnvironmentWidgetCpuRegister}
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
        margin: 5px;
    }
</style>
