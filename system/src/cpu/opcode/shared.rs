use super::super::CPUInterface;

pub fn stack_push(inter: &mut CPUInterface, value: u8) {
    inter.mem.set_addr(0x0100);
    inter.mem.set_addr_lo(inter.reg.s);

    inter.reg.s -= 1;

    inter.mem.set_data(value);
    inter.mem.write_at_addr();
}

pub fn stack_pull(inter: &mut CPUInterface) -> u8 {
    inter.reg.s += 1;

    inter.mem.set_addr(0x0100);
    inter.mem.set_addr_lo(inter.reg.s);

    inter.mem.read_at_addr()
}
