use super::CPUInterface;

use crate::cpu::opcode::Decoder;
use super::super::opcode::{
    AddressingActions,
    InstructionActions, CycleRef,
    addressing, operations,
};


pub struct CurrentOpcode {
    actions: &'static AddressingActions,
    op: &'static InstructionActions,

    action_i: usize,
}

impl CurrentOpcode {
    pub fn new() -> Self {
        CurrentOpcode {
            actions: &addressing::IMP,
            op: &operations::NOP,

            action_i: std::usize::MAX, //force a self.re_init()
        }
    }

    pub fn re_init(&mut self, actions: &'static AddressingActions, op: &'static InstructionActions) {
        self.actions = actions;
        self.op = op;

        self.action_i = 0;
    }

    pub fn fetch(&mut self, inter: &mut CPUInterface) {
        inter.mem.set_addr(
            inter.reg.pc
        );

        let opcode = inter.mem.read_at_addr();

        let (addr, op) = Decoder::decode(opcode);
        self.re_init(addr, op);

        inter.reg.pc += 1;
    }

    fn unchecked_execute(&mut self, inter: &mut CPUInterface) {
        let mut owned_inserted = inter.next_cycle.take(); // consume and replace with None

        let actions = owned_inserted
            .get_or_insert_with(|| {
                let rt = self.actions[self.action_i];
                self.action_i += 1;

                rt
            });

        if self.action_i == 1 { //always the second cycle is a fetch, even on 1 byte opcodes
            inter.mem.set_addr(inter.reg.pc);
            inter.mem.read_at_addr();

            inter.reg.pc += 1;
        }

        for action in *actions {
            match action {
                CycleRef::Fn(fn_ref) => fn_ref(inter),

                CycleRef::OpHardRef(i) => {
                    self.op[*i as usize](inter)
                }

                CycleRef::OpSoftRef(i) => {
                    let fn_opt = self.op.get(*i as usize);

                    if let Some(fn_ref) = fn_opt {
                        fn_ref(inter);
                    }
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.action_i >= self.actions.len()
    }

    pub fn execute(&mut self, inter: &mut CPUInterface) {
        if self.is_done() {
            self.fetch(inter);

            *inter.next_cycle = None;
        } else {
            self.unchecked_execute(inter);
        }
    }

    pub fn cycle_count(&self) -> usize {
        self.action_i
    }
}