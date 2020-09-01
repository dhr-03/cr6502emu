import {DeviceUpdaters} from "./deviceUpdaters";
import {DeviceSetupFns} from "./deviceSetupFns";

const DeviceId = require(process.env.VUE_APP_SYS_JS_PATH).DeviceId;

import EnvironmentWidgetCpu from "../../components/EnvironmentWidgetCpu";
import EnvironmentWidgetMemMonitor from "../../components/EnvironmentWidgetMemMonitor";
import EnvironmentWidgetPixelScreen from "../../components/EnvironmentWidgetPixelScreen";

const DEVICE_DATA = {
    [DeviceId.CPU]: {
        setupFn: DeviceSetupFns.getEmptyMap,
        updater: DeviceUpdaters.copyAll,

        component: EnvironmentWidgetCpu,

        defaultTitle: "CPU",
    },

    [DeviceId.Rom]: {
        setupFn: DeviceSetupFns.getEmptyMap,
        updater: DeviceUpdaters.memMonitorUpdater,

        component: EnvironmentWidgetMemMonitor,

        defaultTitle: "ROM",
    },

    [DeviceId.Ram]: {
        setupFn: DeviceSetupFns.getEmptyMap,
        updater: DeviceUpdaters.memMonitorUpdater,

        component: EnvironmentWidgetMemMonitor,

        defaultTitle: "RAM",
    },

    [DeviceId.PixelScreen]: {
        setupFn: DeviceSetupFns.getEmptyMap,
        updater: DeviceUpdaters.pixelScreenUpdater,

        component:EnvironmentWidgetPixelScreen,

        defaultTitle: "Pixel Screen",
    }

};

export class DeviceIdTools {
    static getSetupFn(id) {
        return DEVICE_DATA[id].setupFn;
    }

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
