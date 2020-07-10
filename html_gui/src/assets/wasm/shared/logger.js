/*
    ########################## INFO ######################

    This is just a temp modification, this will be part of the assembler refactor/rewrite
*/


export class Logger {
    static workingObj;

    static currentLine;
    static handled;

    static endFn;

    static setup(callback) {
        this.endFn = callback;

        this.reset();
    }

    static reset() {
        this.workingObj = null;
        this.currentLine = undefined;

        this.handled = false;
    }

    static setCurrentLine(line) {
        this.currentLine = line;
    }

    static beginGeneric(msgClass) {
        let title;

        if (!isNaN(this.currentLine)) {
            title = `Ln ${this.currentLine}`;
        } else {
            title = this.currentLine || undefined;
        }

        this.workingObj = {
            type: msgClass,
            title,
            parts: [],
        }

    }

    static beginErr() {
        this.beginGeneric("err");
    }

    static beginWarn() {
        this.beginGeneric("warn");
    }

    static beginInfo() {
        this.beginGeneric("info");
    }

    static write(obj) {
        let container = {
            isCode: false,
            content: String(obj),
        };

        this.workingObj.parts.push(container);
    }

    static writeCode(msg) {
        let container = {
            isCode: true,
            content: String(msg),
        };

        this.workingObj.parts.push(container);
    }

    static endMessage() {
        if (this.workingObj) {
            this.endFn(this.workingObj);

            this.reset();
        }

        this.handled = true;
    }

    static genericMessage(kind, msg) {
        Logger.beginGeneric(kind);
        Logger.write(msg);
        Logger.endMessage();
    }

    static genericExplainedCode(kind, msg1, err, msg2) {
        Logger.beginGeneric(kind);
        Logger.write(msg1);
        Logger.writeCode(err);
        Logger.write(msg2);
        Logger.endMessage();
    }

    static msgHandled() {
        let handled = this.handled;

        this.handled = false;

        return handled;
    }

}
