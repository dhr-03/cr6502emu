<template>
    <div>
        <div
            :id="domId"
            :class="{'uk-modal-container': this.container}"

            ref="container"
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
            @click="toggleModal"
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
    import UIkit from "uikit";

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

        data() {
            return {
                ukModal: null,
            };
        },

        computed: {
            showCloseButton() {
                return this.closeButtonType !== "none";
            },

            closeButtonClass() {
                return `uk-modal-close-${this.closeButtonType}`;
            }
        },

        methods: {
            toggleModal() {
                this.ukModal.toggle();
            },

            showModal() {
                this.ukModal.show();
            },

            hideModal() {
                this.ukModal.hide();

            },
        },

        mounted() {
            // lets make sure that everything is mounted.
            this.$nextTick(_ => {
                this.ukModal = UIkit.modal(this.$refs.container, {
                    escClose: this.escClose,
                    bgClose: this.bgClose,

                    stack: this.allowStack,
                });
            });
        },

        beforeDestroy() {
            this.ukModal.$destroy(true);
        },
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
