const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

const Ajv = require("ajv");
import {ProjectSchema} from "../../assets/schema/project";

import router from "../../router";

const LS_KEY = "cr_projects";
const DEBOUNCE_DURATION = 5 * 1000;

function createAjvInstance() {
    let instance = new Ajv({
        useDefaults: true,
        removeAdditional: true,

    });

    return {
        __ajvInstance: instance,
        __schemaValidator: instance.compile(ProjectSchema),
    }
}

export const ProjectManagerStore = {
    namespaced: true,

    state: {
        ...createAjvInstance(),

        projectsCache: [],

        timeoutSaveToLS: null,
        timeoutSavePrj: null,
    },

    mutations: {
        reloadFromLS(state) {
            state.projectsCache = JSON.parse(localStorage.getItem(LS_KEY)) || [];
        },

        // should this be here?
        saveCacheToLS(state) {
            let serializedProjects = JSON.stringify(state.projectsCache);

            localStorage.setItem(LS_KEY, serializedProjects);
        },


        addProjectFromObject(state, prj) {
            state.projectsCache.push(prj);
        },

        updateProject(state, prj) {
            prj.meta.lastMod = Date.now();

            state.projectsCache = state.projectsCache.filter(p => p.meta.pid !== prj.meta.pid);

            state.projectsCache.push(prj);
        },

        deleteProjectById(state, pid) {
            state.projectsCache = state.projectsCache.filter(prj => prj.meta.pid !== pid);
        },

        clearScheduledProjectSave(state) {
            if (state.timeoutSavePrj != null) {
                clearTimeout(state.timeoutSavePrj);

                state.timeoutSavePrj = null;
            }
        }
    },

    actions: {
        debouncedSaveCacheToLS(context) {
            clearTimeout(context.state.timeoutSaveToLS);

            context.state.timeoutSaveToLS = setTimeout(_ => {
                    context.commit("saveCacheToLS");

                    context.state.timeoutSaveToLS = null;
                }
                , DEBOUNCE_DURATION
            );
        },

        saveProjectFromObject(context, prj, autoSync = true) {
            context.commit("updateProject", prj);

            if (autoSync) {
                context.dispatch("debouncedSaveCacheToLS");
            }
        },

        debouncedSaveProjectFromPromise(context, promise, autoSync = true) {
            clearTimeout(context.state.timeoutSavePrj);

            context.state.timeoutSavePrj = setTimeout(_ => {
                    promise.then(prj => {
                        context.dispatch("saveProjectFromObject", prj, autoSync)
                            .then(_ => context.state.timeoutSavePrj = null);
                    });
                }
                , DEBOUNCE_DURATION
            );
        },


        scheduleCurrentProjectSave(context) {
            context.dispatch("debouncedSaveProjectFromPromise",
                context.dispatch("env/saveProjectState", null, {root: true})
            );
        },


        createNewProject(context, autoSync = true) {
            let newPrj = {
                meta: {
                    created: Date.now(),
                    lastMod: Date.now(),

                    // https://stackoverflow.com/a/8084248
                    pid: Math.random().toString(36).substring(7),
                },

                settings: {
                  targetProgramRomId: 1001,
                },

                devices: [
                    {
                        type: DeviceId.Ram,
                        uid: 1000,

                        start: 0,
                        size: 0x1000,

                        config: {},
                    },

                    {
                        type: DeviceId.Rom,
                        uid: 1001,

                        start: 0x1000,
                        size: 0x1000,

                        config: {},
                    },

                    {
                        type: DeviceId.AsciiIOBuffer,
                        uid: 2002,

                        start: 0x2000,
                        size: 0,

                        config: {},
                    },

                    {
                        type: DeviceId.PixelScreen,
                        uid: 2003,

                        start: 0x3000,
                        size: 0,

                        config: {},
                    },

                ],
            };

            context.state.__schemaValidator(newPrj);

            context.commit("addProjectFromObject", newPrj);

            if (autoSync) {
                context.dispatch("debouncedSaveCacheToLS");
            }

            return newPrj.meta.pid;
        },

        async loadProjectFromId(context, id) {
            context.commit("env/setStatusInitializing", null, {root: true});

            let prj = context.getters.getProjectById(id);

            if (prj != null) {
                let schemaIsValid = context.state.__schemaValidator(prj);

                if (schemaIsValid) {
                    //save current

                    await context.dispatch("env/resetToCleanState", null, {root: true});

                    await context.dispatch("env/loadProjectUnchecked", prj, {root: true});

                    context.commit("env/setStatusIdle", null, {root: true});

                } else {
                    let userMessage = "Failed to load project: Invalid schema.";

                    console.error(userMessage, context.state.__schemaValidator.errors);

                    context.commit("env/setInitErrorMessage", userMessage, {root: true});
                    context.commit("env/setStatusInitFailed", null, {root: true});
                }

            } else {
                await router.push({
                    name: "404"
                });
            }
        },

        async saveCurrentProject(context) {
            let currentPrj = await context.dispatch("env/saveProjectState", null, {root: true});
            if (currentPrj.meta.pid && context.rootGetters["env/successfulInitialize"]) {
                context.commit("updateProject", currentPrj);
            }
        },

        tryToImportProject(context, prj) {
            let isValid = context.state.__schemaValidator(prj);

            if (!isValid) {
                console.error("Failed to import project: Validation failed:",
                    context.getters.schemaValidationErrMsg(),
                    context.state.__schemaValidator.errors
                );

                return {
                    ok: false,
                    err: "validate",
                }
            }

            if (context.getters.getProjectById(prj.meta.pid) != null) {
                return {
                    ok: false,
                    err: "exists",
                }
            }

            context.commit("addProjectFromObject", prj);
            context.dispatch("debouncedSaveCacheToLS");

            return {
                ok: true,
            }

        },

        async beforeShutdown(context) {
            if (context.state.timeoutSavePrj) {
                clearTimeout(context.state.timeoutSavePrj);

                // There is a small possibility that this timeout belongs to some other project.
                await context.dispatch("saveCurrentProject");
            }

            // If we just saved a project we also need to save to LS
            if (context.state.timeoutSaveToLS || context.state.timeoutSavePrj) {
                clearTimeout(context.state.timeoutSaveToLS);

                context.commit("saveCacheToLS");

                context.state.timeoutSavePrj = null;
                context.state.timeoutSaveToLS = null;
            }
        },

        completelyDeleteProjectById(context, pid) {
            context.commit("deleteProjectById", pid);

            if (context.state.timeoutSavePrj) {
                clearTimeout(context.state.timeoutSavePrj);
                context.state.timeoutSavePrj = null;
            }

            context.dispatch("debouncedSaveCacheToLS");
        }
    },

    getters: {
        getAllProjects(state) {
            return state.projectsCache;
        },

        getProjectById(_state, getters) {
            return id => getters.getAllProjects.find(prj => prj.meta.pid === id);
        },

        isSafeToShutdown(state) {
            return state.timeoutSavePrj == null && state.timeoutSaveToLS == null;
        },

        schemaValidationErrMsg(state) {
            return _ => state.__ajvInstance.errorsText(
                state.__schemaValidator.errors
            );
        }
    }
}
