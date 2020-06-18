use super::CPUInterface;

use crate::cpu::opcode::Decoder;
use super::super::opcode::{
    AddressingActions,
    AnnotatedOpcode,
    addressing, operations,
};


pub struct CurrentOpcode {
    actions: &'static AddressingActions,

    op: AnnotatedOpcode,

    action_i: usize,
}

impl CurrentOpcode {
    pub fn new() -> Self {
        CurrentOpcode {
            actions: &addressing::IMP,
            op: operations::NOP,

            action_i: std::usize::MAX, //force a self.re_init()
        }
    }

    pub fn re_init(&mut self, actions: &'static AddressingActions, op: AnnotatedOpcode) {
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

        let addr_action = owned_inserted
            .get_or_insert_with(|| {
                let rt = self.actions[self.action_i];
                self.action_i += 1;

                rt
            });


        addr_action(inter, self.op.0, self.op.1);
    }

    pub fn is_done(&self, inter: &mut CPUInterface) -> bool {
        self.action_i >= self.actions.len() &&
            inter.next_cycle.is_none()
    }

    pub fn force_is_done(&mut self) {
        self.action_i = std::usize::MAX;
    }

    pub fn execute(&mut self, inter: &mut CPUInterface) {
        if self.is_done(inter) {
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