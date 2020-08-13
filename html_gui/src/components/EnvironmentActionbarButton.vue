<template>
    <li
        class="cr-actionbar-button cr-animate-click"
        :class="{[colorClass]: true, 'cr-disabled': !buttonEnabled, 'cr-active': buttonActive}"

        :title="explanation"
        @click="clickHandler"
    >
        <font-awesome-icon :icon="iconName"/>

        <slot></slot>
    </li>
</template>

<script>
    const COLOR_PROP = {
        type: String,
        default: _ => "green",
        validator: value => ["green", "yellow", "red", "gray"].includes(value)
    };

    export default {
        name: "EnvironmentActionbarButton",

        props: {
            icon: {
                type: String,
                required: true
            },

            iconActive: {
                type: String,
                default: _ => ""
            },

            colorName: COLOR_PROP,

            colorNameActive: COLOR_PROP,

            explanation: {
                type: String,
                default: _ => ""
            },

            enabled: {
                type: Boolean,
                default:
                    true
            },

            active: {
                type: Boolean,
                default:
                    false
            },

            onClick: {
                type: Function,
                default:
                    _ => {
                    },
            }
        },

        data() {
            return {
                buttonEnabled: this.enabled,
                buttonActive: this.active
            };
        },

        computed: {
            colorClass() {
                const colorName = this.buttonActive ? (this.colorNameActive || this.colorName) : (this.colorName);
                return "cr-" + colorName;
            },

            iconName() {
                return this.buttonActive ? (this.iconActive || this.icon) : (this.icon);
            },
        },

        methods: {
            clickHandler() {
                if (this.enabled) {
                    this.onClick();
                }
            }
        },

        watch: {
            enabled(val) {
                this.buttonEnabled = val;
            },

            active(val) {
                this.buttonActive = val;
            }
        },
    }
</script>

<style lang="less" scoped>
    @import "../assets/less/namedColors";

    .cr-animate-click {
        transition: background 0.7s;
    }

    .cr-animate-click:active:not(.cr-disabled) {
        background-color: #555 !important;
        transition: background 0s;
        transition-timing-function: ease-in-out;
    }

    .cr-actionbar-button {
        padding: 1em 0.3em;

        border-top: 2px solid transparent;
        border-bottom: 2px solid transparent;

        user-select: none;

        border-radius: 2pt;

        &:not(.cr-disabled) {
            cursor: pointer;
        }

        &.cr-disabled {
            cursor: not-allowed;
        }

        &.cr-active {
            background: darken(@oc-gray-9, 10%);

            border-bottom: 2px solid;
        }

    }

    .cr-disabled {
        filter: grayscale(100%);
        opacity: 0.5;
    }
</style>
