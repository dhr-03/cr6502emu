import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetPixelScreen from "../../../components/EnvironmentWidgetPixelScreen";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedPixelScreen extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(start, end, uid);

        this.canvas = null;
    }


    static get type() {
        return DeviceId.PixelScreen;
    }

    static get widgetComponent() {
        return EnvironmentWidgetPixelScreen;
    }

    static get niceName() {
        return "Pixel Screen";
    }

    static get hasFixedSize() {
        return true;
    }

    static get needsExplicitUpdates() {
        return false;
    }


    setupWidget(pkg) {
        this.widget.displayData.canvas = pkg.get("canvas");
    }

    updateWidget(pkg) {
        //doNothing();
    }
}
