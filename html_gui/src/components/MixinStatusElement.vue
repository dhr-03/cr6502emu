<script>
    const TYPE_ICON_MAP = {
        "info": "info-circle",
        "warn": "exclamation-triangle",
        "err": "times-circle"
    };

    export default {
        name: "MixinStatusElement",

        props: {
            type: {
                type: String,
                required: true,
                validator: value => ["info", "warn", "err"].includes(value)
            },
        },

        computed: {
            typeClass: function () {
                return "cr-" + this.type;
            },

            iconName: function () {
                return TYPE_ICON_MAP[this.type];
            }
        }

    }
</script>
