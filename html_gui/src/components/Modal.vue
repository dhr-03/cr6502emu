<template>
    <div>
        <div
            :id="domId"
            :class="computedClassObj"

            :uk-modal="ukModalSettings"
        >
            <div class="uk-modal-dialog uk-overflow-auto">
                <font-awesome-icon
                    v-if="showCloseButton"
                    :class="closeButtonClass"

                    style="font-size: 2em"
                    icon="times"
                />


                <div
                    v-if="showHeader"

                    class="uk-modal-header"
                >
                    <slot name="header"></slot>
                </div>

                <div
                    :uk-overflow-auto="!allowOverflow"

                    class="uk-modal-body"
                >
                    <slot name="body"></slot>
                </div>

                <div
                    v-if="showFooter"

                    class="uk-modal-footer"
                >
                    <slot name="footer"></slot>
                </div>
            </div>
        </div>

        <a
            :href="toggleAction"

            uk-toggle
        >
            <slot
                name="toggle"
            >
                <button>Toggle</button>
            </slot>
        </a>
    </div>
</template>

<script>
    export default {
        name: "Modal",

        props: {
            closeButtonType: {
                type: String,
                default: "none",

                validator: val => ["default", "outside", "none"].indexOf(val) !== -1,
            },

            escClose: {
                type: Boolean,
                default: true,
            },

            bgClose: {
                type: Boolean,
                default: true,
            },

            showHeader: {
                type: Boolean,
                default: true,
            },

            showFooter: {
                type: Boolean,
                default: true,
            },

            center: {
                type: Boolean,
                default: true,
            },

            container: {
                type: Boolean,
                default: false,
            },

            allowOverflow: {
                type: Boolean,
                default: false,
            },

        },

        data: function () {
            return {
                showModal: this.startOpen,
                domId: Date.now().toString(24),
            }
        },

        computed: {
            ukModalSettings() {
                return `esc-close: ${this.escClose}; bg-close: ${this.bgClose};`;
            },

            computedClassObj() {
                return {
                    "uk-margin-auto-vertical": this.center,
                    "uk-modal-container": this.container,
                }
            },


            toggleAction() {
                return `#${this.domId}`;
            },


            showCloseButton() {
                return this.closeButtonType !== "none";
            },

            closeButtonClass() {
                return `uk-modal-close-${this.closeButtonType}`;
            }
        }
    }
</script>

<style lang="less" scoped>

</style>
