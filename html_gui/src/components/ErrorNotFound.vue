<template>
    <div class="cr-container" id="err">
        <p>404</p>

        <router-link :to="{name: 'Home'}">
            Go Home
            <font-awesome-icon icon="external-link-square-alt"/>
        </router-link>
    </div>
</template>

<script>
    export default {
        name: "ErrorNotFound"
    }
</script>

<style lang="less" scoped>
    #err {
        text-align: center;

        & p {
            font-size: 45vh;
        }
    }
</style>
