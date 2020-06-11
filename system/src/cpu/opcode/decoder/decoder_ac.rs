use super::Decoder;

use super::super::{InstructionActions, DecodedInstruction, AddressingActions};
use super::super::behaviour::{operations, addressing};

mod __op {
    use super::{InstructionActions, operations};

    use operations::*;

    //NOP == invalid opcode
    pub const OPCODES_AC: [[&InstructionActions; 8]; 3] = [
        [&NOP, &BIT, &JMP, &NOP, &STY, &LDY, &CPY, &CPX], //addr_mode 0

        [&ORA, &AND, &EOR, &ADC, &STA, &LDA, &CMP, &SBC], //addr_mode 1

        [&ASL, &ROL, &LSR, &ROR, &STX, &LDX, &DEC, &INC] //addr_mode 0
    ];
}

use __op::OPCODES_AC;

mod __addr {
    use super::{AddressingActions, addressing};

    use addressing::*;

    pub const ADDR_MODES_BC: [[&AddressingActions; 8]; 2] = [
        [&IMM, &ZP_, &A__, &ABS, &ZPY, &ZPX, &ABY, &ABX],
        [&IDX, &ZP_, &IMM, &ABS, &IND, &ZPX, &ABY, &ABX]
    ];
}

use __addr::ADDR_MODES_BC;

const NONE: u8 = 0;
const A_A: u8 = 1 << 2;
const A_IMM: u8 = 1 << 0;
const A_ZP: u8 = 1 << 1;
const A_ZPX: u8 = 1 << 5;
const A_ZPY: u8 = 1 << 4;
const A_ABS: u8 = 1 << 3;
const A_ABSX: u8 = 1 << 7;
const A_ABSY: u8 = 1 << 6;

const B_IMM: u8 = 1 << 2;
const B_ZP: u8 = 1 << 1;
const B_ZPX: u8 = 1 << 5;
const B_ABS: u8 = 1 << 3;
const B_ABSX: u8 = 1 << 7;
const B_ABSY: u8 = 1 << 6;
const B_INXD: u8 = 1 << 4;
const B_INDX: u8 = 1 << 0;

const ADDR_MASK_AC: [[u8; 8]; 3] = [
    // ######### GROUP 0 #########
    [
        NONE,
        A_ZP | A_ABS,
        A_ABS,
        A_ABS,
        A_ZP | A_ABS | A_ZPX,
        A_IMM | A_ZP | A_ABS | A_ZPX | A_ABSX,
        A_IMM | A_ZP | A_ABS,
        A_IMM | A_ZP | A_ABS
    ],

    // ######### GROUP 1 #########
    [
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX,
        B_IMM | B_ZP | B_ZPX | B_ABS | B_ABSX | B_ABSY | B_INXD | B_INDX
    ],

    // ######### GROUP 2 #########
    [
        A_A | A_ZP | A_ZPX | A_ABS | A_ABSX,
        A_A | A_ZP | A_ZPX | A_ABS | A_ABSX,
        A_A | A_ZP | A_ZPX | A_ABS | A_ABSX,
        A_A | A_ZP | A_ZPX | A_ABS | A_ABSX,
        A_ZP | A_ZPY | A_ABS,
        A_IMM | A_ZP | A_ZPX | A_ZPY | A_ABS | A_ABSX | A_ABSY,
        A_ZP | A_ZPX | A_ABS | A_ABSX,
        A_ZP | A_ZPX | A_ABS | A_ABSX
    ],
];

impl Decoder {
    pub fn decode_c(a: u8, b: u8, c: u8) -> DecodedInstruction {
        let mapped_b;
        if c == 2 && (a == 4 || a == 5) && (b == 5 || b == 7) { //LDX/STX ZPX/ABSX -> ZPY/ABSY
            mapped_b = b - 1
        } else {
            mapped_b = b;
        }

        let a_usize = a as usize;
        let c_usize = c as usize;

        let opcode = match OPCODES_AC[c_usize].get(a_usize) {
            Some(v) => *v,
            None => &operations::NOP
        };

        let mask = **ADDR_MASK_AC[c_usize]
            .get(a_usize)
            .get_or_insert(&NONE);


        //c=0 and c=2 share the same table, map if necessary. c=3 is invalid
        let bank = (c & 0b01) as usize;
        let addr_mode = *ADDR_MODES_BC[bank]
            .get(mapped_b as usize)
            .unwrap(); //b == 0bXXX


        if mask & (1 << a) != 0 {
            (
                addr_mode,
                opcode
            )
        } else {
            (
                &addressing::IMP,
                &operations::NOP
            )
        }
    }
}