use super::{CPUInterface, CPUOperationManager};
use super::super::{
    register::RegisterContainer,
    opcode::AddressingFn,
};

use crate::system::MemManager;

use crate::dev::{DeviceTrait, DeviceId, utils};

use wasm_bindgen::JsValue;
use js_sys::Map;

pub struct CPU {
    reg: RegisterContainer,
    initial_pc: u16,

    opcode: CPUOperationManager,

    extra_cycle: Option<AddressingFn>,

    // we only have access to MemManager in Self::tick_with_mem, but also need it Self::in update_widget
    bus_value_widget_cache: (u16, u8, bool),
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: RegisterContainer::new(),
            initial_pc: 0,

            opcode: CPUOperationManager::new(),

            extra_cycle: None,

            bus_value_widget_cache: (0, 0, true),
        }
    }

    pub fn tick_with_mem(&mut self, mem_ref: &mut MemManager) {
        let mut inter = CPUInterface {
            mem: mem_ref,

            reg: &mut self.reg,

            next_cycle: &mut self.extra_cycle,

            target_is_mem: true,
        };

        self.opcode.execute(&mut inter);

        self.bus_value_widget_cache.0 = mem_ref.addr();
        self.bus_value_widget_cache.1 = mem_ref.data();
        self.bus_value_widget_cache.2 = mem_ref.rw();
    }

    pub fn operation_is_done(&self) -> bool {
        self.opcode.operation_is_done(&self.extra_cycle)
    }

    pub fn set_initial_pc(&mut self, value: u16) {
        self.initial_pc = value;
    }
}

impl DeviceTrait for CPU {
    #[cfg(debug_assertions)]
    fn tick(&mut self) {
        panic!("Disabled method. Use CPU::tick_with_mem instead.")
    }

    fn reset_system(&mut self) {
        self.reg.reset();
        self.opcode.force_is_done();
        self.extra_cycle = None;

        self.bus_value_widget_cache = (0, 0, true);


        self.reg.pc = self.initial_pc;
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }

    fn setup_widget(&mut self, pkg: &Map) {
        self.update_widget(pkg);
    }

    fn update_widget(&mut self, pkg: &Map) {
        // see crate::cpu::register::RegisterContainer for more info.

        utils::js_map_add_entry_f64(pkg, "a", self.reg.a);
        utils::js_map_add_entry_f64(pkg, "x", self.reg.x);
        utils::js_map_add_entry_f64(pkg, "y", self.reg.y);
        utils::js_map_add_entry_f64(pkg, "p", self.reg.p);

        utils::js_map_add_entry_f64(pkg, "pc", self.reg.pc);
        utils::js_map_add_entry_f64(pkg, "s", self.reg.s);

        utils::js_map_add_entry_f64(pkg, "busAddr", self.bus_value_widget_cache.0);
        utils::js_map_add_entry_f64(pkg, "busData", self.bus_value_widget_cache.1);
        pkg.set(&JsValue::from("busRw"), &JsValue::from(self.bus_value_widget_cache.2));
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::CPU
    }
}
