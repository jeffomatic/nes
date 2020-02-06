use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn inc(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev.wrapping_add(1);
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_inc() {
    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0xFE);
    inc(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0xFF);
    inc(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0);
    inc(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 1);
    inc(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 2);
    assert_eq!(cpu.regs.p, 0);
}

pub fn inx(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.x.wrapping_add(1);
    cpu.regs.x = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_inx() {
    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0xFE;
    inx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0xFF;
    inx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0;
    inx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 1;
    inx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 2);
    assert_eq!(cpu.regs.p, 0);
}

pub fn iny(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.y.wrapping_add(1);
    cpu.regs.y = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_iny() {
    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0xFE;
    iny(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0xFF;
    iny(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0;
    iny(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 1;
    iny(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 2);
    assert_eq!(cpu.regs.p, 0);
}

pub fn dec(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev.wrapping_sub(1);
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_dec() {
    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 2);
    dec(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 1);
    dec(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0);
    dec(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0xFF);
    dec(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0xFE);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn dex(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.x.wrapping_sub(1);
    cpu.regs.x = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_dex() {
    let mut cpu = Cpu::new_test();
    cpu.regs.x = 2;
    dex(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 1;
    dex(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0;
    dex(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0xFF;
    dex(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFE);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn dey(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.y.wrapping_sub(1);
    cpu.regs.y = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_dey() {
    let mut cpu = Cpu::new_test();
    cpu.regs.y = 2;
    dey(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 1;
    dey(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0;
    dey(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0xFF;
    dey(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFE);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
