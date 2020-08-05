// The easiest way to send an Object from rust is using Maps, but vue doesn't react to them, so we need to convert them.

export const DeviceUpdaters = {
    copyAll(widget, pkg) {
        Object.assign(widget.displayData, Object.fromEntries(pkg));
    },

    unimplemented() {
        console.warn("Unimplemented device widget updater.")
    }
}
