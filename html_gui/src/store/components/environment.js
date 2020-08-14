import {DeviceIdTools} from "../../assets/js/deviceIdTools";

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
            execute: false,
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

        devices: [],

        messages: [],
    },

    mutations: {
        __setAsm(state, obj) {
            state.wasm.assembler = obj;
        },

        __setSys(state, obj) {
            state.wasm.system = obj;
        },


        __setDevices(state, obj) {
            state.devices = obj;
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
        },
    },

    actions: {
        setup(context, callback) {

            function setupWasmAsm() {
                return asmLib.default(process.env.VUE_APP_ASM_WASM_PATH).then(
                    wasm => {
                        let assembler = new asmLib.Assembler();
                        assembler.memory = wasm.memory;

                        asmLib.set_panic_hook();

                        context.commit("__setAsm", assembler);
                    }
                );
            }

            function setupWasmSys() {
                return sysLib.default(process.env.VUE_APP_SYS_WASM_PATH).then(
                    wasm => {
                        let system = new sysLib.System();
                        system.memory = wasm.memory;

                        sysLib.set_panic_hook();

                        context.commit("__setSys", system);
                    }
                );
            }


            function updateStatusIfSetupOk() {
                context.commit("currentStatus", EnvironmentState.INITIALIZING);
            }

            function executeUserCallback() {
                if (typeof callback === "function") {
                    callback();
                }
            }


            function onSetupError(err) {
                console.error("Failed to setup env: ", err);

                context.commit("currentStatus", EnvironmentState.FAILED_TO_INIT);
            }


            Promise.resolve(null)
                .then(setupWasmAsm)
                .then(setupWasmSys)

                .then(updateStatusIfSetupOk)
                .then(executeUserCallback)

                .catch(onSetupError);
        },

        initialize(context) {
            const DeviceId = sysLib.DeviceId;

            context.dispatch("purgeAndReloadDeviceCache");

            context.dispatch("addDeviceWithWidget", {type: DeviceId.Ram, start: 0, end: 0x1000});
            context.dispatch("addDeviceWithWidget", {type: DeviceId.Rom, start: 0x1000, end: 0x1000});

            context.commit("currentStatus", EnvironmentState.IDLE);
        },


        buildToRom(context) {
            context.commit("resetMessages");

            const asm = context.getters.__assembler;
            const sys = context.getters.__system;

            let romIndex = 2; //TMP
            let ptr = sys.device_data_ptr_by_index(romIndex);
            let {size, start} = context.state.devices[romIndex];

            let program = document.querySelector("#editor").innerText;
            let romData = new Uint8Array(sys.memory.buffer, ptr, size);

            let success = asm.assemble(program, romData, start);

            context.dispatch("updateAllDevicesWidgets");
            context.commit("buildStatus", success);
        },

        resetSystem(context) {
            context.getters.__system.reset_system();

            context.dispatch("updateAllDevicesWidgets");
        },

        toggleRun(context) {
            const DEFAULT_DELAY_MS = 10;

            function executeUntilStopped() {
                setTimeout(_ => {
                    context.dispatch("systemExecuteOperation");

                    if (context.getters.isRunning) {
                        executeUntilStopped();
                    }
                }, DEFAULT_DELAY_MS);
            }

            if (context.getters.isRunning) {
                context.commit("currentStatus", EnvironmentState.IDLE);
            } else {
                context.dispatch("resetSystem");

                executeUntilStopped();

                context.commit("currentStatus", EnvironmentState.RUNNING);
            }
        },

        toggleDebug(context) {
            if (context.getters.isDebugging) {
                context.commit("currentStatus", EnvironmentState.IDLE);
            } else {
                context.dispatch("resetSystem");

                context.commit("currentStatus", EnvironmentState.DEBUGGING);
            }
        },

        systemTick(context) {
            context.getters.__system.tick();

            context.dispatch("updateAllDevicesWidgets");
        },

        systemExecuteOperation(context) {
            context.getters.__system.execute_operation();

            context.dispatch("updateAllDevicesWidgets");
        },


        purgeAndReloadDeviceCache(context, updateWidgets = true) {
            let sys = context.getters.__system;
            let newCache = [];

            let index = 0;
            let device = sys.device_representation_by_index(0);

            while (device !== undefined) {
                newCache.push(device);
                if (updateWidgets) {
                    // we need to update the devices widgets before they are committed (and thus rendered)
                    // as the widget data is an empty object on creation and trying to access it could lead to exceptions.
                    context.dispatch("updateDeviceWidgetByIndexAndRepr", {index, device});
                }

                index++;

                device = sys.device_representation_by_index(index);
            }

            context.commit("__setDevices", newCache);
        },


        addDeviceWithWidget(context, {type, start, end, uid}) {
            let actualUid = uid || DeviceIdTools.getRandomUID();

            let success = context.getters.__system.add_device_with_uid(type, start, end, actualUid);

            if (success) {
                let newDevIndex = context.state.devices.length; // assuming we are synchronized with rust

                let newDev = context.getters.__system.device_representation_by_index(newDevIndex);

                context.dispatch("setupDeviceWidgetByIndexAndRepr", {device: newDev, index: newDevIndex});

                context.state.devices.push(newDev);
            }

            return success;
        },


        __doDeviceWidgetUpdate(context, {index, device, pkg}) {
            let handler = DeviceIdTools.getUpdater(device.type);

            let getMemFn = function () {
                let ptr = context.getters.__system.device_data_ptr_by_index(index);

                return new Uint8Array(context.getters.__system.memory.buffer, ptr, device.size);
            }

            handler(device.widget, pkg, getMemFn)
        },


        updateDeviceWidgetByIndex(context, index) {
            let device = context.state.devices[index];

            context.dispatch("updateDeviceWidgetByIndexAndRepr", {index, device});
        },

        updateDeviceWidgetByIndexAndRepr(context, {index, device}) {
            let updatePackage = context.getters.__system.device_widget_update_by_index(index);

            context.dispatch("__doDeviceWidgetUpdate", {index, device, pkg: updatePackage});
        },


        setupDeviceWidgetByIndexAndRepr(context, {index, device, updateWidget = true}) {
            let setupHandler = DeviceIdTools.getSetupFn(device.type);
            let setupPackage = setupHandler(device);

            let updatePackage = context.getters.__system.device_widget_setup_by_index(index, setupPackage);

            if (updateWidget) {
                context.dispatch("__doDeviceWidgetUpdate", {index, device, pkg: updatePackage});
            }
        },

        updateAllDevicesWidgets(context) {
            for (let i = 0; i < context.state.devices.length; i++) {
                context.dispatch("updateDeviceWidgetByIndex", i);
            }
        }

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
            return !(state.lock.execute || getters.isDebugging) && getters.isBuilt;
        },

        ableToDebug(state, getters) {
            return !(state.lock.execute || getters.isRunning) && getters.isBuilt;
        },

        ableToStep(state, getters) {
            return getters.isDebugging;
        },

        ableToConfig(state, getters) {
            return !(state.lock.config || getters.isExecuting);
        },


        deviceList(state) {
            return state.devices;
        }

    }
}
