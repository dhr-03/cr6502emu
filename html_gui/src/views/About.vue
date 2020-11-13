<template>
    <div class="crg-container crg-mg-top">
        <p>
            {{ $t("about.description") }}
        </p>

        <a href="https://github.com/dhreck/cr6502emu">
            <font-awesome-icon icon="code"/>
            {{ $t("about.gitLinkText") }}
        </a>
    </div>
</template>

<script>
    export default {
        name: "About"
    }
</script>
