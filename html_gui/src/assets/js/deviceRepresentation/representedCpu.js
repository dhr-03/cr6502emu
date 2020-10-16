import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetCpu from "../../../components/EnvironmentWidgetCpu";
const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedCpu extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(start, end, uid);

        this.widget.displayData.a = null;
        this.widget.displayData.x = null;
        this.widget.displayData.y = null;
        this.widget.displayData.p = null;

        this.widget.displayData.pc = null;
        this.widget.displayData.s = null;


        this.widget.displayData.busAddr = null;
        this.widget.displayData.busData = null;
    }


    static get type() {
        return DeviceId.CPU;
    }

    static get widgetComponent() {
        return EnvironmentWidgetCpu;
    }

    static get niceName() {
        return "CPU";
    }

    static get hasFixedSize() {
        return true;
    }

    static get needsExplicitUpdates() {
        return true;
    }


    setupWidget() {
        this.updateWidget();
    }

    updateWidget() {
        // Vue doesnt support Maps for reactivity.
        Object.assign(this.widget.displayData, Object.fromEntries(this.updatePkg))
    }
}
