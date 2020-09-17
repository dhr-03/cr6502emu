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
            state.projectsCache = state.projectsCache.filter(prj => prj.meta.pid !== prj.meta.id);

            state.projectsCache.push(prj);
        }
    },

    actions: {
        debouncedSaveCacheToLS(context) {
            clearTimeout(context.state.timeoutSaveToLS);

            context.state.timeoutSaveToLS = setTimeout(_ =>
                    context.commit("saveCacheToLS")
                , DEBOUNCE_DURATION
            );
        },


        debouncedSaveProjectFromGetter(context, getterFn, autoSync = true) {
            clearTimeout(context.state.timeoutSavePrj);

            context.state.timeoutSavePrj = setTimeout(_ =>
                    context.commit("updateProject", getterFn())
                , DEBOUNCE_DURATION
            );

            if (autoSync) {
                context.dispatch("debouncedSaveCacheToLS");
            }
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

                devices: {},
            };

            context.commit("addProjectFromObject", newPrj);

            if (autoSync) {
                context.dispatch("debouncedSaveCacheToLS");
            }

            return newPrj.meta.pid;
        },
    },

    getters: {
        getAllProjects(state) {
            return state.projectsCache;
        },

        getProjectById(_state, getters) {
            return id => getters.getAllProjects.find(prj => prj.meta.id === id);
        }
    },
}
