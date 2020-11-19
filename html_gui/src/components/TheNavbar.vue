<template>
    <nav class="crl-navbar uk-navbar uk-navbar-container uk-navbar-transparent">
        <div class="uk-navbar-left">
            <span class="crl-navbar-logo uk-navbar-item uk-logo">Cr6502Emu</span>

            <ul class="uk-navbar-nav">
                <BaseNavbarItem target="Home" :match-exact="true"/>
                <BaseNavbarItem
                    :target="{name: 'Project', params: {pid: currentProjectIdOrPlaceholder}}"
                    :clickable="projectLoaded"
                />
                <BaseNavbarItem target="About" :match-exact="true"/>
            </ul>
        </div>

        <div class="uk-navbar-right uk-margin-right">
            <ul class="uk-navbar-nav">
                <li>
                    <div class="crl-lang-button uk-flex uk-flex-middle">
                        <font-awesome-icon icon="language" class="crl-icon-lang"/>
                        <font-awesome-icon icon="caret-down" class="crl-icon-arrow"/>
                    </div>

                    <div uk-dropdown="mode: click">
                        <ul class="uk-nav uk-dropdown-nav">
                            <li
                                class="uk-nav-header"
                                v-t="'navbar.lang'"
                            />
                            <li
                                v-for="(locale, index) in getAvailableLocales"
                                :key="index"

                                :class="{'uk-active': getCurrentLocale === locale}"

                                @click="_ => setAndSaveLocale(locale)"
                            >
                                <a class="uk-text-uppercase">
                                    {{ locale }}
                                </a>
                            </li>
                        </ul>
                    </div>

                </li>
            </ul>
        </div>
    </nav>
</template>

<script>
    import BaseNavbarItem from "./BaseNavbarItem"
    import {mapActions, mapGetters} from "vuex";

    export default {
        name: "TheNavbar",
        components: {BaseNavbarItem},

        methods: {
            ...mapActions("global", [
                "setAndSaveLocale",
            ]),
        },

        computed: {
            ...mapGetters("env", [
                "currentProjectId"
            ]),

            ...mapGetters("global", [
                "getCurrentLocale",
                "getAvailableLocales",
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

    .crl-navbar {
        background: @oc-gray-8;
    }

    .crl-navbar-logo {
        min-height: 0 !important;
    }

    .crl-lang-button {
        cursor: pointer;
    }

    .crl-icon-lang {
        font-size: 150%;
    }

    .crl-icon-arrow {
        margin-left: 0.3em;
    }

</style>
