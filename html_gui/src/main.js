import Vue from "vue"
import App from "./App.vue"
import router from "./router"
import store from "./store"

import "../node_modules/uikit/dist/js/uikit.min"

import {library} from "@fortawesome/fontawesome-svg-core"
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome"
import {faCode, faExternalLinkSquareAlt} from "@fortawesome/free-solid-svg-icons"

library.add(faCode, faExternalLinkSquareAlt);

Vue.component("font-awesome-icon", FontAwesomeIcon)


Vue.config.productionTip = false

new Vue({
    router,
    store,
    render: h => h(App)
}).$mount("#app")
