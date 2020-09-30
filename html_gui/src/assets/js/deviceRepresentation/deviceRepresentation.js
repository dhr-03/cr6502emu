import {DeviceWidget} from "../deviceWidget";
import Tools from "../tools";

export class DeviceRepresentation {
    constructor(start, end, uid) {
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

    }


    // The device type. (wasmSys.DeviceId)
    static get type() {
        return null;
    }

    static get widgetComponent() {
        return null;
    }

    static get niceName() {
        return null;
    }

    // Some devices have a fixed size, others let the user set it.
    static get hasFixedSize() {
        return null;
    }

    // Some devices need to exchange data with wasm to update, while others do it automatically.
    static get needsExplicitUpdates() {
        return null;
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

    getRepresentationString(numericBase=10) {
        let addrStart = this.start.toString(numericBase);
        let addrEnd = this.end.toString(numericBase);

        return `${this.constructor.niceName} - Id: ${this.uid} - Range: [${addrStart} - ${addrEnd}]`;
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


    setWidgetConfig(newConfig) {
        this._widget._config = newConfig;
    }

    getExportRepresentation() {
        return {
            type: this.constructor.type,
            uid: this._uid,

            start: this._start,
            size: this.constructor.hasFixedSize ? 0 : this.size,

            config: Tools.deepClone(this._widget.config),
        }
    }
}
