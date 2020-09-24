<script>
    import {mapGetters} from "vuex";

    export default {
        name: "MixinSettingsPage",

        computed: mapGetters("env", [
            "projectMeta",
            "projectSettings",
            "deviceList",
            "deviceListWithoutCpu",
        ]),
    }
</script>
