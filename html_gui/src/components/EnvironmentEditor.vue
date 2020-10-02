<template>
    <pre
        id="editor"
        class="language-asm6502"
        data-linenumber="1"
        spellcheck="false"

        :contenteditable="editable"

        ref="editor"
    >{{ initialCode }}</pre>
</template>

<script>
    import "../assets/vendor/prism/prism.js" //prism core
    import "../assets/js/asm6502-lang.js" //prism semi-custom lang


    //convert prism into an editor
    import "../assets/vendor/bililite/range/bililiteRange.js"
    import "../assets/vendor/bililite/range/bililiteRange.undo.js"
    import "../assets/vendor/bililite/range/bililiteRange.util.js"
    import "../assets/vendor/bililite/range/bililiteRange.fancytext.js"

    //fix line numbers feature broken by the scripts above
    import "../assets/vendor/bililite/prism.linenumber.js"

    export default {
        name: "EnvironmentEditor",

        props: {
            editable: {
                type: Boolean,
                default: true
            },

            initialCode: {
                type: String,
                default: "",
            },
        },

        mounted: function () {
            let editor = this.$refs.editor;

            editor = bililiteRange.fancyText(editor, Prism.highlightElement);

            // add the undo's
            editor.addEventListener('keydown', function (evt) {
                if (evt.ctrlKey && evt.which === 90) bililiteRange.undo(evt); // control z
                if (evt.ctrlKey && evt.which === 89) bililiteRange.redo(evt); // control y
            });

            bililiteRange(editor).data().autoindent = true;
        }
    }
</script>

<style lang="css" scoped>
    @import "../assets/vendor/prism/prism.css";

    #editor {
        box-sizing: border-box;
        height: 60vh;

        padding: 1em;
        margin: 0 !important;

        font-weight: 900;
    }

    /* lines fix */
    [class*="language-"] {
        counter-reset: linenumber;
    }

    #editor >>> span.line {
        display: inline-block;
        width: 100%;
        counter-increment: linenumber;
    }

    #editor >>> span.line:before {
        content: counter(linenumber);
        width: 1em;
        display: inline-block;

        color: #999;
        border-right: 1px solid;

        margin-right: 0.8em;
        padding-right: 0.8em;
    }
</style>
