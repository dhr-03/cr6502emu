export default {
    U16MaxValue: (2 ** 16) - 1, // unsigned int 16

    deepClone(source) {
        //apparently this is how you do  deep clone in js in a few lines according to the MDN.
        return JSON.parse(JSON.stringify(source));
    },

    // too lazy to implement it myself
    // https://stackoverflow.com/a/3177838
    //
    // Update: heavily modified to work with i18n
    timeSince(date) {
        let seconds = Math.floor((Date.now() - date) / 1000);

        const amounts = [
            [31536000, "year"],
            [2592000, "month"],
            [86400, "day"],
            [3600, "hour"],
            [60, "minute"]
        ];

        for (let [divider, name] of amounts) {
            let interval = seconds / divider;

            if (interval > 1) {
                return {
                    unit: name,
                    amount: Math.floor(interval),
                };
            }
        }

        return {
            unit: "second",
            amount: seconds
        }

    },

    getRandomUID() {
        return Math.floor(
            Math.random() * this.U16MaxValue
        );
    }
}
