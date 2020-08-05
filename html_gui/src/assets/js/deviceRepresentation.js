export class DeviceRepresentation {
    constructor(type, start, end, uid) {
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
        this._widget = {};
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
}
