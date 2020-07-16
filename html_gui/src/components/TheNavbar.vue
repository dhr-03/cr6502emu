<template>
    <nav class="cr-navbar uk-navbar uk-navbar-container uk-navbar-transparent">
        <div class="uk-navbar-left">
            <span class="cr-navbar-logo uk-navbar-item uk-logo">CrDemo</span>

            <ul class="uk-navbar-nav">
                <BaseNavbarItem target="Home" :match-exact="true"/>
                <BaseNavbarItem :target="{name: 'Project', params: {pid: '_'}}" :clickable="false"/>
                <BaseNavbarItem target="About"/>
            </ul>
        </div>
    </nav>
</template>

<script>
    import BaseNavbarItem from "./BaseNavbarItem"

    export default {
        name: "TheNavbar",
        components: {BaseNavbarItem},
    }
</script>

<style lang="less" scoped>
    @import "~open-color/open-color.less";

    .cr-navbar {
        background: @oc-gray-8;
    }

    .cr-navbar-logo {
        min-height: 0!important;
    }

</style>
