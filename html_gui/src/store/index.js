import Vue from "vue"
import Vuex from "vuex"

Vue.use(Vuex);

import {EnvironmentStore} from "./components/environment"

export default new Vuex.Store({
    modules: {
        env: EnvironmentStore
    }
})
