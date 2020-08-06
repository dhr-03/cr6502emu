import {DeviceUpdaters} from "./deviceUpdaters";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

import EnvironmentWidgetCpu from "../../components/EnvironmentWidgetCpu";
import EnvironmentWidgetMemMonitor from "../../components/EnvironmentWidgetMemMonitor";

const DEVICE_DATA = {
    [DeviceId.CPU]: {
        updater: DeviceUpdaters.copyAll,
        component: EnvironmentWidgetCpu,

        defaultTitle: "CPU",
    },

    [DeviceId.Rom]: {
        updater: DeviceUpdaters.memMonitorUpdater,
        component: EnvironmentWidgetMemMonitor,

        defaultTitle: "ROM",
    },

    [DeviceId.Ram]: {
        updater: DeviceUpdaters.memMonitorUpdater,
        component: EnvironmentWidgetMemMonitor,

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

    static getRandomUID() {
        const MAX_VALUE = (2 ** 16) - 1; // unsigned int 16

        return Math.floor(
            Math.random() * MAX_VALUE
        );
    }
}
