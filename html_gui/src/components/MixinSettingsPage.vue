<script>
    import {mapGetters} from "vuex";

    export default {
        name: "MixinSettingsPage",

        niceName: "Setting",

        computed: mapGetters("env", [
            "projectMeta",
            "projectSettings",
            "deviceList",
            "deviceListWithoutCpu",
        ]),
    }
</script>
