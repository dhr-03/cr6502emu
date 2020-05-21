export class Logger {
    static baseNode;
    static workingNode;
    static writeNode;
    static currentLine;

    static handled;

    static setup(selector) {
        this.baseNode = document.querySelector(selector);

        this.reset();
    }

    static reset() {
        this.workingNode = null;
        this.writeNode = null;
        this.currentLine = -1;

        this.handled = false;
    }

    static setCurrentLine(line) {
        this.currentLine = line;
    }


    static beginGeneric(msgClass) {
        let template = `
            <div class="message ${msgClass}">
                <div style="width: 8em;float: left;border-right: 2px solid; margin-right: 1em">
                    <span>line: ${this.currentLine !== undefined ? this.currentLine : "?"}</span>
                </div>
                <div class="msg-content"></div>
            </div>

        `;

        let node = document.createElement("div");
        node.innerHTML = template;

        this.workingNode = node;
        this.writeNode = node.querySelector(".msg-content");

    }

    static beginInfo() {
        this.beginGeneric("info");
    }

    static beginWarn() {
        this.beginGeneric("warn");
    }

    static beginErr() {
        this.beginGeneric("err");
    }

    static write(obj) {
        let node = document.createElement("span");

        node.innerText = obj;
        this.writeNode.appendChild(node);
    }

    static writeCode(msg) {
        let node = document.createElement("code");
        node.innerText = msg;

        this.writeNode.appendChild(node);

    }

    static endMessage() {
        if (this.workingNode) {
            this.baseNode.appendChild(this.workingNode);

            if (this.baseNode.classList.contains("err")) {
                this.handled = true;
            }

            this.workingNode = null;
            this.writeNode = null;
        }
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