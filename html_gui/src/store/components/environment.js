const asmLib = require(process.env.VUE_APP_ASM_JS_PATH);
const sysLib = require(process.env.VUE_APP_SYS_JS_PATH);

export const EnvironmentState = {
    SETTING_UP: 0,
    INITIALIZING: 1,
    FAILED_TO_INIT: 2,

    IDLE: 10,

    RUNNING: 20,
    DEBUGGING: 21,
}

export const EnvironmentStore = {
    namespaced: true,

    state: {
        lock: {
            build: false,
            reset: false,
            run: false,
            config: false,
        },

        status: {
            buildSuccessful: false,
            currentAction: EnvironmentState.SETTING_UP,
        },

        wasm: {
            assembler: null,
            system: null,
        },

        messages: [],
    },

    mutations: {
        __setAsm(state, obj) {
            state.wasm.assembler = obj;
        },

        __setSys(state, obj) {
            state.wasm.system = obj;
        },


        buildStatus(state, value) {
            state.status.buildSuccessful = value;
        },

        currentStatus(state, value = EnvironmentState.IDLE) {
            state.status.currentAction = value;
        },


        addMessage(state, value) {
            state.messages.push(value);
        },

        resetMessages(state) {
            state.messages = [];
        }
    },

    actions: {
        setup(context, callback) {
            asmLib.default(process.env.VUE_APP_ASM_WASM_PATH).then(
                wasm => {
                    let assembler = new asmLib.Assembler();
                    assembler.memory = wasm.memory;

                    asmLib.set_panic_hook();

                    context.commit("__setAsm", assembler);
                }
            )
                .then(
                    _ => {
                        return sysLib.default(process.env.VUE_APP_SYS_WASM_PATH).then(
                            wasm => {
                                let system = new sysLib.System();
                                system.memory = wasm.memory;

                                sysLib.set_panic_hook();

                                context.commit("__setSys", system);
                            }
                        )
                    })

                .then(
                    _ => context.commit("currentStatus", EnvironmentState.INITIALIZING)
                )

                .then(
                    _ => {
                        if (typeof callback === "function") {
                            callback();
                        }
                    }
                )

                .catch(
                    err => {
                        console.error("Failed to setup env: ", err);

                        context.commit("currentStatus", EnvironmentState.FAILED_TO_INIT);
                    }
                )
        },

        initialize(context) {
            const sys = context.getters.__system;

            const DeviceId = sysLib.DeviceId;
            sys.add_device(DeviceId.Ram, 0, 0x1000);
            sys.add_device(DeviceId.Rom, 0x1000, 0x1000);


            context.commit("currentStatus", EnvironmentState.IDLE);
        },


        buildToRom(context) {
            context.commit("resetMessages");

            const asm = context.getters.__assembler;
            const sys = context.getters.__system;

            let romId = 1;
            let ptr = sys.device_data_ptr(romId);
            let size = sys.device_size(romId);

            let program = document.querySelector("#editor").innerText;
            let rom = new Uint8Array(sys.memory.buffer, ptr, size);

            let success = asm.assemble(program, rom);

            context.commit("buildStatus", success);
        },

        resetSystem(context) {
            context.getters.__system.reset_system();
        },

        toggleRun(context) {
            if (context.getters.isRunning) {
                context.commit("currentStatus", EnvironmentState.IDLE);
            } else {
                context.commit("currentStatus", EnvironmentState.RUNNING);
            }
        },

        toggleDebug(context) {
            if (context.getters.isDebugging) {
                context.commit("currentStatus", EnvironmentState.IDLE);
            } else {
                context.commit("currentStatus", EnvironmentState.DEBUGGING);
            }
        },

        cpuShortStep(context) {
            context.getters.__system.tick();
        },

    },

    getters: {
        __assembler(state) {
            return state.wasm.assembler;
        },

        __system(state) {
            return state.wasm.system;
        },


        messagesList(state) {
            return state.messages;
        },


        isInitializing(state) {
            return (
                state.status.currentAction === EnvironmentState.SETTING_UP ||
                state.status.currentAction === EnvironmentState.INITIALIZING
            );
        },

        successfulInitialize(state) {
            return state.status.currentAction !== EnvironmentState.FAILED_TO_INIT;
        },


        isRunning(state) {
            return state.status.currentAction === EnvironmentState.RUNNING;
        },

        isDebugging(state) {
            return state.status.currentAction === EnvironmentState.DEBUGGING;
        },

        isExecuting(state, getters) {
            return getters.isRunning || getters.isDebugging;
        },

        isBuilt(state) {
            return state.status.buildSuccessful;
        },


        ableToBuild(state, getters) {
            return !(state.lock.build || getters.isExecuting);
        },

        ableToReset(state, getters) {
            return !(state.lock.reset || getters.isExecuting) && getters.isBuilt;
        },

        ableToRun(state, getters) {
            return !(state.lock.run || getters.isDebugging) && getters.isBuilt;
        },

        ableToDebug(state, getters) {
            return !(state.lock.run || getters.isRunning) && getters.isBuilt;
        },

        ableToStep(state, getters) {
            return getters.isDebugging;
        },

        ableToConfig(state, getters) {
            return !(state.lock.config || getters.isExecuting);
        },

    }
}
