// The easiest way to send an Object from rust is using Maps, but vue doesn't react to them, so we need to convert them.

export const DeviceUpdaters = {
    copyAll(widget, pkg) {
        Object.assign(widget.displayData, Object.fromEntries(pkg));
    },

    async memMonitorUpdater(widget, pkg, memArrayBuilder) {
        if (pkg.get("update")) {
            if (widget.displayData.memArray === undefined) {
                widget.displayData.memArray = memArrayBuilder();
            } else {
                // Vue doesnt support typed arrays for reactivity, so we need to force it.
                let oldValue = widget.displayData.memArray;
                widget.displayData.memArray = []
                widget.displayData.memArray = oldValue;

            }
        }
    },

    unimplemented() {
        console.warn("Unimplemented device widget updater.")
    }
}
