const WasmMsgs = require(process.env.VUE_APP_ASM_JS_PATH).LoggerMessage;

const LangEn = {
    router: {
        Home: "Home",
        Project: "Project",
        About: "About",

        "404": "404",
    },

    navbar: {
        lang: "Language",
    },

    projectChooser: {
        title: {
            openLocal: "Open Local Project",

            noSavedProjects: "No saved projects found.",
        },

        button: {
            new: "New Project",
            import: "Import Project",
        },

        importPrompt: {
            title: "Upload Project",
            selectButtonText: "Select File",

            error: {
                title: "Failed to import file:",

                invalidData: "Invalid Data.",
                unavailableId: "A project with the id {pid} already exists.",
                unknown: "Unknown error.",

                invalidFile: "Invalid file structure.",
            },
        },
    },

    environment: {
        noRomSelected: "No Rom Selected",

        initialize: {
            initializing: "Initializing",
            failed: "Failed to initialize Environment",
        },

        actionbar: {
            build: "Build",
            reset: "Reset",

            run: "Run",

            debug: "Debug",
            debugCycle: "Execute Cycle",
            debugInstruction: "Execute Instruction",

            settings: "Settings",
        },

        settings: {
            // Using vue component name:
            EnvironmentSettingPrjMeta: {
                niceName: "Project about",

                prjName: "Project Name",
                prjId: "Project Id",

                dateLastMod: "Last project activity",
                dateCreated: "Project creation date"

            },

            EnvironmentSettingPrjFile: {
                niceName: "File",

                downloadPrj: "Download Project",
                deletePrj: "Delete Project",

                deletePrompt: {
                    nameTitle: "Delete project:",
                    idTitle: "Id:"
                },
            },

            EnvironmentSettingEnvInterface: {
                niceName: "Interface",

                preferredNumBase: "Preferred numeric base",
                maxMonitorRows: "Maximum Memory Monitor rows",
                maxMonitorRowsExplanation: "(This can impact performance and loading times)",

            },

            EnvironmentSettingPrjDevices: {
                niceName: "Devices",

                table: {
                    uid: "UID",
                    type: "Type",
                    start: "Start",
                    end: "End",
                    size: "Size",
                    actions: "Actions",

                    emptyMessage: "No devices found.",
                },

                addPrompt: {
                    title: "Add Device",

                    failedToAdd: "Failed to add device.",

                    numBase: "Numeric Base",

                    dev: {
                        type: "Device Type",
                        addr: "Device Address",
                        size: "Size",
                    },

                    button: {
                        toggle: "Add device",
                        submit: "Add Device",
                    },

                },

                swapPrompt: {
                    title: "Swap devices",

                    failedToSwap: "Failed to swap devices.",

                    dev: {
                        a: "Device A",
                        b: "Device B",

                        select: "Select One",
                    },

                    button: {
                        toggle: "Swap devices",
                        submit: "Swap devices",
                    },
                },

                buildRom: {
                    title: "Build Rom",

                    emptyMessage: "None found",
                    nonEmptyMessage: "Select one"
                },

            },

            EnvironmentSettingEnvRunMode: {
                niceName: "Run Mode",

                explanation: {
                    a: "Due to some technical difficulties, cr-emu can only execute ASM code every 4ms-10ms (approx).",
                    b: "The following setting determines how many opcodes are executed at once in every cycle (execute and update widgets).",
                    c: "Be aware that while the cycle is running the whole web page will freeze.",

                },

                operationsPerCycle: "Operations per cycle",
            },
        },

        widget: {
            cpu: {
                flags: {
                    negative: "Negative",
                    overflow: "Overflow",
                    zero: "Zero",
                    carry: "Carry",
                },

                items: {
                    a: "A",
                    x: "X",
                    y: "Y",

                    pc: "PC",
                    s: "Stack Ptr",
                    flags: "Flags",

                    bus: {
                        addr: "Bus Addr",
                        data: "Bus Data"
                    },
                },
            },

            asciiBuffer: {
                writeHere: "Write here ...",
            },
        },

        logbar: {
            title: "Status Log",

            emptyMessage: "Wow, such empty"
        },
    },

    about: {
        description: "cr6502emu is an assembler and emulator of the MOS 6502 CPU.",
        gitLinkText: "Github Repo",
    },

    guiCommon: {
        button: {
            save: "Save",
            delete: "Delete",
            cancel: "Cancel",
        },

        numericBase: {
            hex: "Hexadecimal",
            dec: "Decimal",
        },

        wait: "Wait ...",
    },

    timeAgo: {
        template: "{amount} {unit} ago",

        unit: {
            second: "second | seconds",
            minute: "minute | minutes",
            hour: "hour | hours",
            day: "day | days",
            month: "month | months",
            year: "year | years",
        },
    },

    wasmAsm: {
        [WasmMsgs.PrsErrNumParse]: "Failed to parse value {code} as uint 8/uint 16",
        [WasmMsgs.PrsErrNumParseI8]: "Failed to parse value {code} as int 8",

        [WasmMsgs.PrsErrExpectedZP]: "Expected 1 byte, found 2",


        [WasmMsgs.AsmErrLblNeverDef]: "Label {code} is never defined",
        [WasmMsgs.AsmErrLblReDef]: "Label {code} has already been defined",
        [WasmMsgs.AsmErrLblShort]: "Label {code} it's too short",
        [WasmMsgs.AsmErrLblLong]: "Label {code} it's too long",

        [WasmMsgs.AsmErrAsmFailed]: "Assemble failed",
        [WasmMsgs.AsmErrRomTooSmall]: "The program ROM is too small",

        [WasmMsgs.AsmErrUnknownOpcode]: "Unknown opcode",
        [WasmMsgs.AsmErrAddrMode]: "Opcode: {code} is incompatible with {code2}",
        [WasmMsgs.AsmErrTargetTooFar]: "The target it's not in the -128-127 range",

        [WasmMsgs.AsmErrEmptyInput]: "Nothing to assemble",

        [WasmMsgs.AsmInfoAsmSuccess]: "Assembled into {code} bytes",


        [WasmMsgs.McrErrNonAscii]: "Non ascii chars found",
        [WasmMsgs.McrErrNumParse]: "Failed to parse {code} as a number",


        [WasmMsgs.PreUnknownOpcode]: "Unknown opcode",
        [WasmMsgs.PreUnknownMacro]: "Unknown macro",
        [WasmMsgs.PreUnknownPattern]: "Unknown pattern",
        [WasmMsgs.PreUnknownIdentifier]: "Unknown identifier",

        [WasmMsgs.PreUnknownAddressingMode]: "Unknown addressing mode",
        [WasmMsgs.PreWrongAddressingMode]: "Wrong addressing mode",

        [WasmMsgs.PreInvalidValue]: "Invalid value",
        [WasmMsgs.PreValueSize]: "Value size",

        [WasmMsgs.PreSyntaxError]: "Syntax error",

    },
};

export default LangEn;
