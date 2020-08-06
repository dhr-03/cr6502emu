<template>
    <div class="crl-mem-monitor">
        <div
            v-for="([address, row], indexRow) in iterableRows"
            :key="indexRow"
        >

            <EnvironmentNumberContainer
                class="crl-address"

                :value="address"

                :length-in-bytes="2"
                :base="16"
            />

            <div class="crl-value-row">
                <EnvironmentNumberContainer
                    v-for="(cell, indexCell) in row"
                    :key="indexCell"

                    :value="cell"

                    :base="16"
                />
            </div>
        </div>
    </div>
</template>

<script>
    import MixinEnvironmentWidget from "./MixinEnvironmentWidget";
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";

    export default {
        name: "EnvironmentWidgetMemMonitor",
        components: {EnvironmentNumberContainer},
        mixins: [MixinEnvironmentWidget],

        computed: {
            deviceSize() {
                return this.widget.displayData.memArray.length;
            },

            rowCount() {
                return Math.ceil(
                    this.deviceSize / 8
                );
            },

            cappedRowCount() {
                return Math.min(
                    this.rowCount, 50
                )
            },

            iterableRows() {
                let rows = []

                for (let i = 0; i < this.cappedRowCount; i++) {
                    let sliceStart = i * 8;
                    let sliceEnd = Math.min(sliceStart + 8, this.deviceSize - 1);

                    rows.push([
                            this.device.start + sliceStart,
                            this.widget.displayData.memArray.slice(sliceStart, sliceEnd)
                        ]
                    );
                }

                return rows;
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .crl-mem-monitor {
        background: @oc-gray-7;
        padding: 0.5em;

        font-size: 78%;
    }

    .crl-address {
        color: #999;

        font-weight: bold;
    }

    .crl-value-row {
        display: inline-block;

        padding-left: .5em;
        border-left: 1px solid #999;

        color: #fff;
    }
</style>
