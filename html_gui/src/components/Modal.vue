<template>
    <div>
        <div
            :id="domId"
            :class="{'uk-modal-container': this.container}"

            :uk-modal="ukModalSettings"
        >
            <div
                :class="{'uk-margin-auto-vertical': this.center, [this.contentClass]: this.contentClass}"
                class="crl-dark-modal uk-modal-dialog uk-overflow-auto"
            >
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
            domId: {
                type: String,
                default: _ => Date.now().toString(24),
            },

            contentClass: {
                type: String,
                required: false,
            },

            closeButtonType: {
                type: String,
                default: "default",

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

            allowStack: {
                type: Boolean,
                default: false,
            },

        },

        computed: {
            ukModalSettings() {
                return `esc-close: ${this.escClose}; bg-close: ${this.bgClose};stack: ${this.allowStack}`;
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
    @import "../../node_modules/open-color/open-color";

    .crl-dark-modal {
        background: @oc-gray-9;
        color: #fff;

        .uk-modal-header, .uk-modal-footer {
            background: lighten(@oc-gray-9, 5%);
        }
    }
</style>
