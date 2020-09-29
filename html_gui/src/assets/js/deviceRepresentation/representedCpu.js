import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetCpu from "../../../components/EnvironmentWidgetCpu";
const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedCpu extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(DeviceId.CPU, start, end, uid, true, true);

        this.widget.displayData.a = null;
        this.widget.displayData.x = null;
        this.widget.displayData.y = null;
        this.widget.displayData.p = null;

        this.widget.displayData.pc = null;
        this.widget.displayData.s = null;


        this.widget.displayData.busAddr = null;
        this.widget.displayData.busData = null;
    }

    setupWidget(pkg) {
        this.updateWidget(pkg);
    }

    updateWidget(pkg) {
        // Vue doesnt support Maps for reactivity.
        Object.assign(this.widget.displayData, Object.fromEntries(pkg))
    }

    get widgetComponent() {
        return EnvironmentWidgetCpu;
    }

    get niceName() {
        return "CPU";
    }
}
