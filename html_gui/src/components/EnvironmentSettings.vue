<template>
    <div class="crl-inherit-mh uk-grid uk-grid-collapse uk-light">
        <div class="crl-inherit-mh crl-scrollable uk-width-1-5">

            <div
                v-for="(component, index) of settingsComponents"
                :key="index"

                class="crl-submenu uk-text-truncate"
                :class="{'selected': component === activeComponent}"

                @click="updateActiveComponent(component)"
            >
                {{component}}
            </div>

        </div>

        <div class="crl-setting-container uk-width-expand">
            <h3>{{ activeComponent }}</h3>
            <hr>

            <component :is="activeComponent"/>
        </div>
    </div>
</template>

<script>
    export default {
        name: "EnvironmentSettings",
        components: {},

        data() {
            return {
                settingsComponents: [],
                activeComponent: "",
            }
        },

        methods: {
          updateActiveComponent(to) {
              if (this.activeComponent !== to) {
                  this.activeComponent = to;
              }
          }
        },
    }
</script>

<style lang="less" scoped>
    .crl-inherit-mh {
        max-height: inherit;
    }

    .crl-scrollable {
        overflow: auto;
    }

    .crl-submenu {
        padding: 0.5em;

        border-top: 1px solid #555;
        border-bottom: 1px solid #555;

        &.selected {
            color: white;
            font-weight: bold;
        }
    }

    .crl-setting-container {
        margin: 1em;
    }

</style>
