export const GlobalStore = {
    namespaced: true,

    state: {
        i18nInstance: null,
    },

    mutations: {
        __saveLocaleToLS(state) {
            localStorage.cr_lang = state.i18nInstance.locale;
        },

        setI18n(state, instance) {
            state.i18nInstance = instance;
        },

        setLocale(state, locale) {
            state.i18nInstance.locale = locale;
        }
    },

    actions: {
        tryLoadLocaleFromLS(context) {
            let locale = localStorage.cr_lang;
            let validLocales = context.getters["getAvailableLocales"];

            if (locale && validLocales.indexOf(locale) !== -1) {
                context.state.i18nInstance.locale = locale;
            }
        },

        setAndSaveLocale(context, locale) {
            context.commit("setLocale", locale);
            context.commit("__saveLocaleToLS");
        },
    },

    getters: {
        getCurrentLocale(state) {
            return state.i18nInstance.locale;
        },

        getAvailableLocales(state) {
            return state.i18nInstance.availableLocales;
        }
    }
}
