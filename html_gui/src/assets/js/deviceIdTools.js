const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

const DEVICE_DATA = {
    [DeviceId.CPU]: {
        updater: null,
        component: null,
    },

    [DeviceId.Rom]: {
        updater: null,
        component: null,
    },

    [DeviceId.Ram]: {
        updater: null,
        component: null,
    },

};

export class DeviceIdTools {
    static getUpdater(id) {
        return DEVICE_DATA[id].updater;
    }

    static getWidgetComponent(id) {
        return DEVICE_DATA[id].component;
    }
}
