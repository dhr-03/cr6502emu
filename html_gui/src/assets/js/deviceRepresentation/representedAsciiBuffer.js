import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetAsciiIOBuffer from "../../../components/EnvironmentWidgetAsciiIOBuffer";
const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedAsciiBuffer extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(DeviceId.AsciiIOBuffer, start, end, uid, true, false);

        this.widget.displayData.in = null;
        this.widget.displayData.out = null;
    }

    setupWidget(pkg) {
        this.widget.displayData.in = pkg.get("in");
        this.widget.displayData.out = pkg.get("out");
    }

    updateWidget(pkg) {
        //doNothing();
    }

    get widgetComponent() {
        return EnvironmentWidgetAsciiIOBuffer;
    }

    get niceName() {
        return "Ascii IO Buffer";
    }
}
