export class DeviceWidget {
    constructor(config) {
        this._config = config != null ? config : {};
        this._displayData = {};
    }

    get config() {
        return this._config;
    }

    get displayData() {
        return this._displayData;
    }
}
