<template>
    <div class="cr-container cr-mg-top">
        <router-link :to="{name: 'Project', params: {pid: 'test'}}">
            + New Project
        </router-link>
    </div>
</template>

<script>
    export default {
        name: "Home",
    }
</script>
