import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetAsciiIOBuffer from "../../../components/EnvironmentWidgetAsciiIOBuffer";
const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedAsciiBuffer extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(start, end, uid, true, false);

        this.widget.displayData.in = null;
        this.widget.displayData.out = null;
    }


    static get type() {
        return DeviceId.AsciiIOBuffer;
    }

    static get widgetComponent() {
        return EnvironmentWidgetAsciiIOBuffer;
    }

    static get niceName() {
        return "Ascii IO Buffer";
    }

    static get hasFixedSize() {
        return true;
    }

    static get needsExplicitUpdates() {
        return false;
    }


    setupWidget() {
        this.widget.displayData.in = this.updatePkg.get("in");
        this.widget.displayData.out = this.updatePkg.get("out");
    }

    updateWidget() {
        //doNothing();
    }
}
