import {DeviceId} from "../wasm/system";

const U16_MAX = (2 ** 16) - 1; // unsigned int 16

export const ProjectSchema = {
    type: "object",
    properties: {
        meta: {
            type: "object",

            default: {},

            properties: {
                name: {
                    type: "string",
                    default: "Unnamed Project",
                },

                code: {
                    type: ["string", "null"],
                    default: null,
                },

                created: {
                    type: "integer",
                    minimum: 0,
                },

                lastMod: {
                    type: "integer",
                    minimum: 0,
                },

                pid: {
                    type: "string",
                },
            },

            required: [
                "name",
                "code",
                "created",
                "lastMod",
                "pid",
            ],
        },

        settings: {
            type: "object",

            default: {},

            properties: {
                preferredNumericBase: {
                  type: "integer",
                  enum: [10, 16],

                  default: 16,
                },
            },

            required: [
                "preferredNumericBase",
            ],
        },

        devices: {
            type: "array",

            default: [],

            items: {
                type: "object",

                properties: {
                    type: {
                        type: "integer",
                        enum: Object.entries(DeviceId)
                            .map(v => v[1])
                            .filter(v => typeof v === "number" && v !== 255),
                    },

                    uid: {
                        type: "integer",
                        minimum: 0,
                        maximum: U16_MAX,
                    },


                    start: {
                        type: "integer",
                        minimum: 0,
                        maximum: U16_MAX,
                    },

                    size: {
                        type: "integer",
                        minimum: 0,
                        maximum: U16_MAX,
                    },

                    config: {
                        type: "object",
                    },
                },

                required: [
                    "type",
                    "uid",
                    "start",
                    "size",
                    "config",
                ]
            }
        }
    },

    required: [
        "meta",
        "settings",
        "devices",
    ],
}
