const LS_KEY = "cr_projects";
const DEBOUNCE_DURATION = 5 * 1000;

export const ProjectManagerStore = {
    namespaced: true,

    state: {
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
                    });

                    context.state.timeoutSavePrj = null;
                }
                , DEBOUNCE_DURATION
            );
        },

        createNewProject(context, autoSync = true) {
            let newPrj = {
                meta: {
                    name: "Unnamed Project",
                    code: null, //use environment default


                    created: Date.now(),
                    lastMod: Date.now(),

                    // https://stackoverflow.com/a/8084248
                    pid: Math.random().toString(36).substring(7),
                },

                settings: {},

                devices: [],
            };

            context.commit("addProjectFromObject", newPrj);

            if (autoSync) {
                context.dispatch("debouncedSaveCacheToLS");
            }

            return newPrj.meta.pid;
        },

        async beforeShutdown(context) {
            if (context.state.timeoutSavePrj) {
                clearTimeout(context.state.timeoutSavePrj);

                // There is a small possibility that this timeout belongs to some other project.
                let currentPrj = await context.dispatch("env/exportProjectToObject", null, {root: true});
                if (currentPrj.meta.pid) {
                    context.commit("updateProject", currentPrj);
                }
            }

            // If we just saved a project we also need to save to LS
            if (context.state.timeoutSaveToLS || context.state.timeoutSavePrj) {
                clearTimeout(context.state.timeoutSaveToLS);

                context.commit("saveCacheToLS");

                context.state.timeoutSavePrj = null;
                context.state.timeoutSaveToLS = null;
            }
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
        }
    },
}
