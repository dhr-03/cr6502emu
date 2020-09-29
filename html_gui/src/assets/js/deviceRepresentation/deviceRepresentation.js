import {DeviceWidget} from "../deviceWidget";
import Tools from "../tools";

export class DeviceRepresentation {
    constructor(type, start, end, uid, hasFixedSize, needsExplicitUpdates) {
        // The device type. (wasmSys.DeviceId)
        this._type = type;

        // The range used in the address bus by the device.
        //
        // ## WARNING: ##
        // This doesnt mean that the user/cpu can access the hole range, another device might be partially/totally overlapping.
        this._start = start;
        this._end = end;

        // The uid (unique-id) assigned to this device.
        this._uid = uid;

        // The Widget.
        this._widget = new DeviceWidget();

        // Some devices have a fixed size, others let the user set it.
        this._hasFixedSize = hasFixedSize;

        // Some devices need to exchange data with wasm to update, while others do it automatically.
        this._needsExplicitUpdates = needsExplicitUpdates;
    }

    get type() {
        return this._type;
    }


    get start() {
        return this._start;
    }

    get end() {
        return this._end;
    }

    get size() {
        return this.end - this.start;
    }


    get uid() {
        return this._uid;
    }


    get widget() {
        return this._widget;
    }

    get hasFixedSize() {
        return this._hasFixedSize;
    }

    get needsExplicitUpdates() {
        return this._needsExplicitUpdates;
    }


    getWasmSetupPkg() {
        return new Map();
    }

    setupWidget(pkg, memArrayBuilder) {
        console.warn("Unimplemented setup");
    }

    updateWidget(pkg) {
        console.warn("Unimplemented updater");
    }


    get widgetComponent() {
        return null;
    }

    get niceName() {
        return null;
    }


    setWidgetConfig(newConfig) {
        this._widget._config = newConfig;
    }

    getExportRepresentation() {
        return {
            type: this._type,
            uid: this._uid,

            start: this._start,
            size: this._hasFixedSize ? 0 : this.size,

            config: Tools.deepClone(this._widget.config),
        }
    }
}
