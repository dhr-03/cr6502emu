import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetMemMonitor from "../../../components/EnvironmentWidgetMemMonitor";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedRom extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(start, end, uid);

        this.widget.displayData.memArray = null;
    }


    static get type() {
        return DeviceId.Rom;
    }

    static get widgetComponent() {
        return EnvironmentWidgetMemMonitor;
    }

    static get niceName() {
        return "Rom";
    }

    static get hasFixedSize() {
        return false;
    }

    static get needsExplicitUpdates() {
        return true;
    }


    setupWidget(pkg, memArrayBuilder) {
        this.widget.displayData.memArray = memArrayBuilder();
    }

    updateWidget(pkg, memArrayBuilder) {
        if (pkg.get("update")) {
            // Vue doesnt support typed arrays for reactivity, so we need to force an update.
            this.widget.displayData.__ob__.dep.notify();
        }
    }
}
