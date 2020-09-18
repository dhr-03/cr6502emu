import Vue from "vue"
import Vuex from "vuex"

Vue.use(Vuex);

import {EnvironmentStore} from "./components/environment"
import {ProjectManagerStore} from "./components/projectManager";

export default new Vuex.Store({
    modules: {
        env: EnvironmentStore,
        prj: ProjectManagerStore,
    }
})
