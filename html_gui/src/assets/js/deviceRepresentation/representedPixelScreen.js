import {DeviceRepresentation} from "./deviceRepresentation";
import EnvironmentWidgetPixelScreen from "../../../components/EnvironmentWidgetPixelScreen";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedPixelScreen extends DeviceRepresentation {
    constructor(start, end, uid) {
        super(DeviceId.PixelScreen, start, end, uid, true, false);

        this.canvas = null;
    }

    setupWidget(pkg) {
        this.widget.displayData.canvas = pkg.get("canvas");
    }

    updateWidget(pkg) {
        //doNothing();
    }

    get widgetComponent() {
        return EnvironmentWidgetPixelScreen;
    }

    get niceName() {
        return "Pixel Screen";
    }
}
