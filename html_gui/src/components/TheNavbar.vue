<template>
    <nav class="cr-navbar uk-navbar uk-navbar-container uk-navbar-transparent">
        <div class="uk-navbar-left">
            <span class="cr-navbar-logo uk-navbar-item uk-logo">CrDemo</span>

            <ul class="uk-navbar-nav">
                <BaseNavbarItem target="Home" :match-exact="true"/>
                <BaseNavbarItem
                    :target="{name: 'Project', params: {pid: currentProjectIdOrPlaceholder}}"
                    :clickable="projectLoaded"
                />
                <BaseNavbarItem target="About" :match-exact="true"/>
            </ul>
        </div>
    </nav>
</template>

<script>
    import BaseNavbarItem from "./BaseNavbarItem"
    import {mapGetters} from "vuex";

    export default {
        name: "TheNavbar",
        components: {BaseNavbarItem},

        computed: {
            ...mapGetters("env", [
                "currentProjectId"
            ]),

            currentProjectIdOrPlaceholder() {
                return this.currentProjectId || "_";
            },

            projectLoaded() {
                return this.currentProjectId != null;
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "~open-color/open-color.less";

    .cr-navbar {
        background: @oc-gray-8;
    }

    .cr-navbar-logo {
        min-height: 0 !important;
    }

</style>
