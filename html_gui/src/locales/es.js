const WasmMsgs = require(process.env.VUE_APP_ASM_JS_PATH).LoggerMessage;

const LangEs = {
    router: {
        Home: "Home",
        Project: "Proyecto",
        About: "Acerca",

        "404": "404",
    },

    navbar: {
        lang: "Idioma",
    },

    projectChooser: {
        title: {
            openLocal: "Abrir proyecto local",

            noSavedProjects: "No hay proyectos guardados.",
        },

        button: {
            new: "Nuevo proyecto",
            import: "Importar proyecto",
        },

        importPrompt: {
            title: "Importar archivo",
            selectButtonText: "Seleccionar",

            error: {
                title: "No se pudo importar:",

                invalidData: "Archivo invalido.",
                unavailableId: "Ya existe un proyecto con el id {pid}.",
                unknown: "Error desconocido.",

                invalidFile: "Estructura invalida.",
            },
        },
    },

    environment: {
        noRomSelected: "No se ha seleccionado una Rom",

        initialize: {
            initializing: "Inicializando",
            failed: "No se pudo Inicializar.",
        },

        actionbar: {
            build: "Ensamblar",
            reset: "Reiniciar",

            run: "Ejecutar",

            debug: "Debuguear",
            debugCycle: "Ejecutar ciclo",
            debugInstruction: "Ejecutar instruccion",

            settings: "Opciones",
        },

        settings: {
            // Using vue component name:
            EnvironmentSettingPrjMeta: {
                niceName: "Acerca del proyecto",

                prjName: "Nombre",
                prjId: "Id",

                dateLastMod: "Ultima actividad",
                dateCreated: "Fecha de creacion",

            },

            EnvironmentSettingPrjFile: {
                niceName: "Archivo",

                downloadPrj: "Descargar",
                deletePrj: "Eliminar",

                deletePrompt: {
                    nameTitle: "Eliminar proyecto:",
                    idTitle: "Id:",
                },
            },

            EnvironmentSettingEnvInterface: {
                niceName: "Interfaz",

                preferredNumBase: "Base numerica",
                maxMonitorRows: "Maximas filas en el Monitor de Memoria",
                maxMonitorRowsExplanation: "(Puede impactar el rendimiento y tiempos de carga)",

            },

            EnvironmentSettingPrjDevices: {
                niceName: "Dispositivos",

                table: {
                    uid: "UID",
                    type: "Tipo",
                    start: "Inicio",
                    end: "Fin",
                    size: "Tamaño",
                    actions: "Acciones",

                    emptyMessage: "No se encontraron dispositivos.",
                },

                addPrompt: {
                    title: "Agregar dispositivo",

                    failedToAdd: "Ocurrio un error.",

                    numBase: "Base numerica",

                    dev: {
                        type: "Tipo",
                        addr: "Direccion",
                        size: "Tamaño",
                    },

                    button: {
                        toggle: "Agregar",
                        submit: "Agregar",
                    },

                },

                swapPrompt: {
                    title: "Intercambiar dispositivos",

                    failedToSwap: "Ocurrio un error.",

                    dev: {
                        a: "Dispositivo A",
                        b: "Dispositivo B",

                        select: "Seleccionar ...",
                    },

                    button: {
                        toggle: "Intercambiar",
                        submit: "Intercambiar",
                    }
                },

                buildRom: {
                    title: "Rom de programa",

                    emptyMessage: "Ninguna encontrada",
                    nonEmptyMessage: "Seleccionar ...",
                },

            },

            EnvironmentSettingEnvRunMode: {
                niceName: "Ejecucion",

                explanation: {
                    a: "Debido a dificultades tecnicas, cr-emu solo puede ejecutar instrucciones cada 4ms-10ms (approx).",
                    b: "Esta configuracion determina cuantas instrucciones son ejecutadas en cada ciclo (ejecutar y actualizar los widgets).",
                    c: "Ten en cuenta que mientras esta ejecutando, la pagina web no respondera.",

                },

                operationsPerCycle: "Instrucciones por ciclo",
            },
        },

        widget: {
            cpu: {
                flags: {
                    negative: "Negativo",
                    overflow: "Overflow",
                    zero: "Cero",
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
                        addr: "Bus Direc.",
                        data: "Bus Datos",
                    },
                },
            },

            asciiBuffer: {
                writeHere: "Escribe aqui ...",
            },

            memMonitor: {
                cappedMsg: "-- Limitado en Opciones --"
            },
        },

        logbar: {
            title: "Mensajes",

            emptyMessage: "Nada por aqui",
        },
    },

    about: {
        description: "cr6502emu es un ensamblador y emulador de la CPU MOS 6502.",
        gitLinkText: "Repositorio de Github",
    },

    guiCommon: {
        button: {
            save: "Guardar",
            delete: "Eliminar",
            cancel: "Cancelar",
        },

        numericBase: {
            hex: "Hexadecimal",
            dec: "Decimal",
        },

        wait: "Espera ...",
    },

    timeAgo: {
        template: "Hace {amount} {unit}",

        unit: {
            second: "segundo | segundos",
            minute: "minuto | minutos",
            hour: "hora | horas",
            day: "dia | dias",
            month: "mes | meses",
            year: "año | años",
        },
    },

    wasmAsm: {
        [WasmMsgs.PrsErrNumParse]: "El valor {code} no es un uint 8/uint 16 valido",
        [WasmMsgs.PrsErrNumParseI8]: "El valor {code} no es un int 8 valido",

        [WasmMsgs.PrsErrExpectedZP]: "Se esperaba un byte, se encontraron 2",


        [WasmMsgs.AsmErrLblNeverDef]: "La etiqueta {code} nunca es definida",
        [WasmMsgs.AsmErrLblReDef]: "La etiqueta {code} ya fue definida",
        [WasmMsgs.AsmErrLblShort]: "La etiqueta {code} es muy corta",
        [WasmMsgs.AsmErrLblLong]: "La etiqueta {code} es muy corta",

        [WasmMsgs.AsmErrAsmFailed]: "No se pudo ensamblar",
        [WasmMsgs.AsmErrRomTooSmall]: "La ROM de programa es muy chica",

        [WasmMsgs.AsmErrUnknownOpcode]: "Opcode desconocido",
        [WasmMsgs.AsmErrAddrMode]: "El opcode: {code} no es compatible con {code2}",
        [WasmMsgs.AsmErrTargetTooFar]: "La direccion no esta en el rango -128-127 desde la direccion actual",

        [WasmMsgs.AsmErrEmptyInput]: "No hay nada para ensamblar",

        [WasmMsgs.AsmInfoAsmSuccess]: "Ensamblado en {code} bytes",


        [WasmMsgs.McrErrNonAscii]: "Hay que no son ascii",
        [WasmMsgs.McrErrNumParse]: "No se pudo convertir {code} a un numero",


        [WasmMsgs.PreUnknownOpcode]: "Opcode desconocido",
        [WasmMsgs.PreUnknownMacro]: "Macro desconocida",
        [WasmMsgs.PreUnknownPattern]: "Patron desconocido",
        [WasmMsgs.PreUnknownIdentifier]: "Identificador desconocido",

        [WasmMsgs.PreUnknownAddressingMode]: "Modo de direccionamiento desconocido",
        [WasmMsgs.PreWrongAddressingMode]: "Modo de direccionamiento invalido",

        [WasmMsgs.PreInvalidValue]: "Valor invalido",
        [WasmMsgs.PreValueSize]: "Tamaño de valor invalido",

        [WasmMsgs.PreSyntaxError]: "Error de sintaxis",

    },

    wasmSys: {
        unsupported: "Funcion no soportada: {code}",

        unimplemented: "Funcion no implementada: {code}",
    }

};

export default LangEs;
