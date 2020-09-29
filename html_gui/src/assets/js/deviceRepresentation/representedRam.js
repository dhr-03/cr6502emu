import {RepresentedRom} from "./representedRom";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

export class RepresentedRam extends RepresentedRom {
    constructor(start, end, uid) {
        super(start, end, uid);
    }


    static get type() {
        return DeviceId.Ram;
    }

    static get niceName() {
        return "Ram";
    }
}
