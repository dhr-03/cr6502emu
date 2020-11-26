import {Logger} from "../../../../shared/logger";

// https://stackoverflow.com/a/15710692
const hashCode = s => s.split('').reduce((a, b) => {
    a = ((a << 5) - a) + b.charCodeAt(0);
    return a & a
}, 0);

export class CappedLogger {
    static waitUntil = {};

    static cappedExplainedErr(template, code1, code2) {
        let now = Date.now();

        let hash = hashCode(`${template} ${code1} ${code2}`);
        let nextMsgShow = this.waitUntil[hash] || 0;

        if (nextMsgShow <= now) {
            Logger.setCurrentLine("Run");
            Logger.explainedErr(template, code1, code2);

            this.waitUntil[hash] = now + (10 * 1000);
        }
    }
}
