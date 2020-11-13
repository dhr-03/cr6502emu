<template>
    <div class="crg-container crg-mg-top">
        <p
            v-t="'about.description'"
        />

        <a href="https://github.com/dhreck/cr6502emu">
            <font-awesome-icon icon="code"/>
            <span
                class="cr-mg-t"
                v-t="'about.gitLinkText'"
            />
        </a>
    </div>
</template>

<script>
    export default {
        name: "About"
    }
</script>
