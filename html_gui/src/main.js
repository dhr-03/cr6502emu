import Vue from "vue"
import App from "./App.vue"
import router from "./router"
import store from "./store"

import "../node_modules/uikit/dist/js/uikit.min"

import {library} from "@fortawesome/fontawesome-svg-core"
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome"

// commented == imported more than once
import {
    //About
    faCode,

    //Alert
    faInfoCircle, faExclamationTriangle, faTimesCircle,

    //ErrorNotFound
    faExternalLinkSquareAlt,

    //EnvironmentWidget
    faBars,

    //EnvironmentWidgetsHolder
    faPlus,

    //Environment
    /*faCog, faTimesCircle*/

    //EnvironmentActionBar
    faHammer, faSyncAlt, faPlay, faStop, faBug, faChevronRight, faCog,

    //EnvironmentLogBar
    /*faChevronRight,*/ faGhost, faChevronDown,

}
    from "@fortawesome/free-solid-svg-icons"

library.add(
    faCode,
    faExternalLinkSquareAlt,
    faInfoCircle, faExclamationTriangle, faTimesCircle,
    faBars,
    faPlus,
    faCog, faTimesCircle,
    faHammer, faSyncAlt, faPlay, faStop, faBug, faChevronRight, faCog,
    faChevronRight, faGhost, faChevronDown,
);

Vue.component("font-awesome-icon", FontAwesomeIcon)


Vue.config.productionTip = false

new Vue({
    router,
    store,
    render: h => h(App)
}).$mount("#app")
