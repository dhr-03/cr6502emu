<template>
    <div class="crl-mem-monitor">
        <div
            class="uk-grid uk-grid-collapse"

            v-for="([address, row], indexRow) in iterableRows"
            :key="indexRow"
        >

            <EnvironmentNumberContainer
                class="crl-address uk-width-auto"

                :value="address"

                :length-in-bytes="2"
                :base="preferredNumericBase"
            />

            <div class="crl-value-row uk-width-expand uk-flex uk-flex-between">

                    <EnvironmentNumberContainer
                        v-for="(cell, indexCell) in row"
                        :key="indexCell"

                        :value="cell"

                        :base="preferredNumericBase"
                    />

            </div>
        </div>

        <p class="uk-text-center uk-text-bold uk-text-primary">
            -- Capped by Settings --
        </p>
    </div>
</template>

<script>
    import MixinEnvironmentWidget from "./MixinEnvironmentWidget";
    import EnvironmentNumberContainer from "./EnvironmentNumberContainer";
    import MixinPreferredNumericBase from "./MixinPreferredNumericBase";

    export default {
        name: "EnvironmentWidgetMemMonitor",
        mixins: [MixinEnvironmentWidget, MixinPreferredNumericBase],
        components: {EnvironmentNumberContainer},

        computed: {
            deviceSize() {
                return this.widget.displayData.memArray.length;
            },

            valuesPerRow() {
                return this.preferredNumericBase === 16 ? 8 : 5;
            },

            rowCount() {
                return Math.ceil(
                    this.deviceSize / this.valuesPerRow
                );
            },

            maxRows() {
                return this.$store.getters["env/projectSettings"].memoryMonitorMaxRows;
            },

            cappedRowCount() {
                return Math.min(
                    this.rowCount, this.maxRows
                );
            },

            displayedRowsAreCapped() {
              return this.rowCount > this.cappedRowCount;
            },

            iterableRows() {
                let perRow = this.valuesPerRow;
                let rows = []

                for (let i = 0; i < this.cappedRowCount; i++) {
                    let sliceStart = i * perRow;
                    let sliceEnd = Math.min(sliceStart + perRow, this.deviceSize - 1);

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

        //TODO: temp fix
        min-height: 15.5em;

        font-size: 78%;
    }

    .crl-address {
        margin-right: 0.5em;

        color: #999;

        font-weight: bold;
    }

    .crl-value-row {
        padding-left: 0.5em;
        border-left: 1px solid #999;

        color: #fff;
    }
</style>
