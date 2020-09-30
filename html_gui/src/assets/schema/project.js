import {DeviceId} from "../wasm/system";

import Tools from "../js/tools";

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

                memoryMonitorMaxRows: {
                    type: "integer",
                    minimum: 1,
                    maximum: 500,

                    default: 50,
                },

                targetProgramRomId: {
                    type: ["integer", "null"],
                    minimum: 0,
                    maximum: Tools.U16MaxValue,

                    default: null,
                },
            },

            required: [
                "preferredNumericBase",
                "memoryMonitorMaxRows",
                "targetProgramRomId",
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
                        maximum: Tools.U16MaxValue,
                    },


                    start: {
                        type: "integer",
                        minimum: 0,
                        maximum: Tools.U16MaxValue,
                    },

                    size: {
                        type: "integer",
                        minimum: 0,
                        maximum: Tools.U16MaxValue,
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
