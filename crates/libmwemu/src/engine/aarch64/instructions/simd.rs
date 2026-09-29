use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode, Operand, SIMDSizeCode};

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.opcode {
        Opcode::MOVI => exec_movi(emu, ins),
        Opcode::FMOV => exec_fmov(emu, ins),
        _ => {
            log::warn!("simd: unhandled opcode {:?}", ins.opcode);
            true
        }
    }
}

fn exec_movi(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.operands[0] {
        Operand::SIMDRegisterElements(_, reg, _) | Operand::SIMDRegister(_, reg) => {
            let imm = match ins.operands[1] {
                Operand::Immediate(v) => v as u128,
                Operand::ImmShift(v, shift) => (v as u128) << shift,
                _ => 0,
            };
            emu.regs_aarch64_mut().v[reg as usize] = imm;
            true
        }
        _ => true,
    }
}

fn exec_fmov(emu: &mut Emu, ins: &Instruction) -> bool {
    match (&ins.operands[0], &ins.operands[1]) {
        (Operand::SIMDRegister(_, rd), Operand::SIMDRegister(_, rn)) => {
            emu.regs_aarch64_mut().v[*rd as usize] = emu.regs_aarch64().v[*rn as usize];
        }
        (Operand::SIMDRegister(sz, rd), Operand::Register(_, rn)) => {
            let val = emu.regs_aarch64().get_x(*rn as usize);
            let v = if matches!(sz, SIMDSizeCode::D) {
                val as u128
            } else {
                (val & 0xffffffff) as u128
            };
            emu.regs_aarch64_mut().v[*rd as usize] = v;
        }
        (Operand::Register(_, rd), Operand::SIMDRegister(sz, rn)) => {
            let v = emu.regs_aarch64().v[*rn as usize];
            let val = if matches!(sz, SIMDSizeCode::D) {
                v as u64
            } else {
                (v & 0xffffffff) as u64
            };
            emu.regs_aarch64_mut().set_x(*rd as usize, val);
        }
        _ => {}
    }
    true
}
