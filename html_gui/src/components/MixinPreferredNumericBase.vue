<script>
    export default {
        name: "MixinPreferredNumericBase",

        computed: {
            preferredNumericBase() {
                return this.$store.getters["env/projectSettings"].preferredNumericBase;
            },
        },
    }
</script>
