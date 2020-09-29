<script>
    import {DeviceRepresentation} from "../assets/js/deviceRepresentation/deviceRepresentation";

    export default {
        name: "MixinEnvironmentWidget",

        props: {
            device: {
                type: DeviceRepresentation,
                required: true
            },
        },

        data() {
            return {
                widget: this.device.widget
            };
        }
    }
</script>
