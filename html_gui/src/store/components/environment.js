export const EnvironmentState = {
    INITIALIZING: 0,
    FAILED_TO_INIT: 1,

    IDLE: 10,

    RUNNING: 20,
    DEBUGGING: 21,
};

export const EnvironmentStore = {
    namespaced: true,

    state: {
        lockBuild: false,
        lockReset: false,
        lockRun: false,
        lockConfig: false,

        buildSuccessful: false,
        status: EnvironmentState.IDLE,

        messages: [],
    },

    mutations: {
        buildStatus(state, value) {
            state.buildSuccessful = value;
        },

        currentStatus(state, value = EnvironmentState.IDLE) {
            state.status = value;
        },


        addMessage(state, value) {
            state.messages.push(value);
        },

        resetMessages(state) {
          state.messages = [];
        }
    },

    actions: {
        buildToRom(context) {
            context.commit("buildStatus", Math.random() >= 0.5);
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
        }


    },

    getters: {
        messagesList(state) {
            return state.messages
        },



        isInitializing(state) {
            return state.status === EnvironmentState.INITIALIZING;
        },

        successfulInitialize(state) {
            return state.status !== EnvironmentState.FAILED_TO_INIT;
        },



        isRunning(state) {
            return state.status === EnvironmentState.RUNNING;
        },

        isDebugging(state) {
            return state.status === EnvironmentState.DEBUGGING;
        },

        isExecuting(state, getters) {
            return getters.isRunning || getters.isDebugging;
        },

        isBuilt(state) {
            return state.buildSuccessful;
        },


        ableToBuild(state, getters) {
            return !(state.lockBuild || getters.isExecuting);
        },

        ableToReset(state, getters) {
            return !(state.lockReset || getters.isExecuting) && getters.isBuilt;
        },

        ableToRun(state, getters) {
            return !(state.lockRun || getters.isDebugging) && getters.isBuilt;
        },

        ableToDebug(state, getters) {
            return !(state.lockRun || getters.isRunning) && getters.isBuilt;
        },

        ableToStep(state, getters) {
            return getters.isDebugging;
        },

        ableToConfig(state, getters) {
            return !(state.lockConfig || getters.isExecuting);
        },

    }
};
