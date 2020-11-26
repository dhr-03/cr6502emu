/*
    ########################## INFO ######################

    This is just a temp modification, this will be part of the assembler refactor/rewrite

    UPDATE: updated to work with i18n, now its worse
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

    static beginGeneric(type, templateId) {
        let title;

        if (!isNaN(this.currentLine)) {
            title = `Ln ${this.currentLine}`;
        } else {
            title = this.currentLine || undefined;
        }

        let fullTemplate = typeof templateId === "string" ? templateId : "wasmAsm." + templateId;

        this.workingObj = {
            type,
            title,
            templateId: fullTemplate,
            codeItems: [],
        }

    }

    static beginErr(template) {
        this.beginGeneric("err", template);
    }

    static beginWarn(template) {
        this.beginGeneric("warn", template);
    }

    static beginInfo(template) {
        this.beginGeneric("info", template);
    }

    static addCode(code) {
        this.workingObj.codeItems.push(code);
    }

    static endMessage() {
        if (this.workingObj) {
            this.endFn(this.workingObj);

            this.reset();
        }

        this.handled = true;
    }

    static genericMessage(kind, template) {
        Logger.beginGeneric(kind, template);
        Logger.endMessage();
    }

    static infoMessage(template) {
        Logger.genericMessage("info", template)
    }

    static warnMessage(template) {
        Logger.genericMessage("warn", template)
    }

    static errMessage(template) {
        Logger.genericMessage("err", template)
    }

    static genericExplainedCode(kind, template, code1, code2) {
        Logger.beginGeneric(kind, template);

        Logger.addCode(code1);

        if (code2 != null) {
            Logger.addCode(code2);
        }

        Logger.endMessage();
    }

    static explainedInfo(template, code1, code2) {
        Logger.genericExplainedCode("info", template, code1, code2);
    }

    static explainedWarn(template, code1, code2) {
        Logger.genericExplainedCode("warn", template, code1, code2);
    }

    static explainedErr(template, code1, code2) {
        Logger.genericExplainedCode("err", template, code1, code2);
    }

    static msgHandled() {
        let handled = this.handled;

        this.handled = false;

        return handled;
    }

}
