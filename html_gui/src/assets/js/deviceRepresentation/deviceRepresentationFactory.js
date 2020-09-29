let MappedDevIdsToTypes = null;

// we need to do this this way, as this file is first imported by rust, so rust types are not available yet.
import(process.env.VUE_APP_SYS_JS_PATH).then(sysLib => {
    const DeviceId = sysLib.DeviceId;

    const RepresentedCpu = require("./representedCpu").RepresentedCpu;
    const RepresentedPixelScreen = require("./representedPixelScreen").RepresentedPixelScreen;
    const RepresentedAsciiBuffer = require("./representedAsciiBuffer").RepresentedAsciiBuffer;
    const RepresentedRom = require("./representedRom").RepresentedRom;
    const RepresentedRam = require("./representedRam").RepresentedRam;

    MappedDevIdsToTypes = {
        [DeviceId.CPU]: RepresentedCpu,

        [DeviceId.PixelScreen]: RepresentedPixelScreen,
        [DeviceId.AsciiIOBuffer]: RepresentedAsciiBuffer,

        [DeviceId.Rom]: RepresentedRom,
        [DeviceId.Ram]: RepresentedRam,
    }
});

export class DeviceRepresentationFactory {
    static new(type, start, end, uid) {
        let constructorClass = MappedDevIdsToTypes[type];

        if (constructorClass) {
            return  new constructorClass(start, end, uid);

        } else {
            throw new Error(`Unknown DeviceId: ${type}`);
        }
    }
}
