<template>
    <div class="cr-widget-cpu">
        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.a')"
            :value="widget.displayData.a"
        />

        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.x')"
            :value="widget.displayData.x"
        />

        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.y')"
            :value="widget.displayData.y"
        />


        <hr class="crl-separator">


        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.pc')"
            :value="widget.displayData.pc"

            :length-in-bytes="2"
        />

        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.s')"
            :value="widget.displayData.s"

            :length-in-bytes="1"
        />

        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.flags')"
        >
            <EnvironmentWidgetCpuFlags
                :value="widget.displayData.p"
            />
        </EnvironmentWidgetCpuRegister>


        <hr class="crl-separator">


        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.bus.addr')"
            :value="widget.displayData.busAddr"

            :pre-value="busMode"
            :length-in-bytes="2"
        />

        <EnvironmentWidgetCpuRegister
            :name="$t('environment.widget.cpu.items.bus.data')"
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

        components: {EnvironmentWidgetCpuFlags, EnvironmentWidgetCpuRegister},

        computed: {
            busMode() {
                return this.widget.displayData.busRw ? "r" : "w";
            }
        }
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
