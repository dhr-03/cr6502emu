import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetMemMonitor from "../../../components/EnvironmentWidgetMemMonitor";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedRom extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(DeviceId.Rom, start, end, uid, false, true);

        this.widget.displayData.memArray = null;
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

    get widgetComponent() {
        return EnvironmentWidgetMemMonitor;
    }

    get niceName() {
        return "ROM";
    }
}
