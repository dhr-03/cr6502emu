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
    },
};

export default LangEn;
