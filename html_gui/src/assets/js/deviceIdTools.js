import {DeviceUpdaters} from "./deviceUpdaters";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

const DEVICE_DATA = {
    [DeviceId.CPU]: {
        updater: DeviceUpdaters.copyAll,
        component: null,

        defaultTitle: "CPU",
    },

    [DeviceId.Rom]: {
        updater: DeviceUpdaters.unimplemented,
        component: null,

        defaultTitle: "ROM",
    },

    [DeviceId.Ram]: {
        updater: DeviceUpdaters.unimplemented,
        component: null,

        defaultTitle: "RAM",
    },

};

export class DeviceIdTools {
    static getUpdater(id) {
        return DEVICE_DATA[id].updater;
    }

    static getWidgetComponent(id) {
        return DEVICE_DATA[id].component;
    }

    static getWidgetDefaultTitle(id) {
        return DEVICE_DATA[id].defaultTitle;
    }
}
