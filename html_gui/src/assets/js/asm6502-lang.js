Prism.languages.asm6502 = {
    'comment': /;.*/,
    'directive': {
        pattern: /\.\w+(?= )/,
        alias: 'keyword'
    },
    'string': /(["'`])(?:\\.|(?!\1)[^\\\r\n])*\1/,
    'opcode': {
        pattern: /\b(?:ADC|AND|ASL|BCC|BCS|BEQ|BIT|BMI|BNE|BPL|BRK|BVC|BVS|CLC|CLD|CLI|CLV|CMP|CPX|CPY|DEC|DEX|DEY|EOR|INC|INX|INY|JMP|JSR|LDA|LDX|LDY|LSR|NOP|ORA|PHA|PHP|PLA|PLP|ROL|ROR|RTI|RTS|SBC|SEC|SED|SEI|STA|STX|STY|TAX|TAY|TSX|TXA|TXS|TYA)\b/,
        alias: 'property'
    },
    'hexnumber': {
        pattern: /#?\$[\da-f]{2,4}\b/i,
        alias: 'string'
    },
    'binarynumber': {
        pattern: /#?%[01]+\b/,
        alias: 'string'
    },
    'decimalnumber': {
        pattern: /#?\b\d+\b/,
        alias: 'string'
    },
    'register': {
        pattern: /\b[xya]\b/i,
        alias: 'variable'
    },

    /* ############### Custom ############### */

    'labeldef': {
        pattern: /\b\w{3,15}:\s*\n/i,
        alias: "function"
    },

    'labelref': {
        pattern: /(?:%|lo |hi )\w{3,15}/i,
        alias: "function"
    },

    'macroparam': {
        pattern: /\b(?:asciiz|byte|bytex2)\b/,
        alias: "boolean"
    },
};
