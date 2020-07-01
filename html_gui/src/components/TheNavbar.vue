<template>
    <nav class="uk-navbar uk-navbar-container uk-navbar-transparent">
        <div class="uk-navbar-left">
            <router-link
                    :to="{name: 'Home'}"
                    class="uk-navbar-item uk-logo"
            >
                cr6502emu
            </router-link>

            <ul class="uk-navbar-nav">
                <BaseNavbarItem target="Home" :match-exact="true"/>
                <BaseNavbarItem :target="{name: 'Project', params: {pid: '_'}}" :clickable="false"/>
                <BaseNavbarItem target="About"/>
            </ul>
        </div>
    </nav>
</template>

<script>
    import BaseNavbarItem from "./BaseNavbarItem";
    export default {
        name: "TheNavbar",
        components: {BaseNavbarItem},
    }
</script>

<style lang="less" scoped>
    @import "~open-color/open-color.less";

    .uk-navbar {
        background: @oc-gray-8;
        margin-bottom: 2em;

    }
</style>
