//! This crate implements an emulator for the
//! [TinyCPU](https://github.com/omikami747/tinycpu), a small CPU
//! designed to be implemented on TTL chips. It is compatible with the
//! default Python emulator, `emu`.

use std::num::Wrapping;

/// Implements the CPU, and keeps track of its state.
pub struct Cpu {
    regs: Registers,
    mem: Vec<Wrapping<u8>>,
}

impl Cpu {
    /// Constructor for the CPU: the current implementation requires a
    /// memory file to be provided, otherwise it panics. For example,
    /// ```
    /// let mut cpu = Cpu::new(Some(mem_init));
    /// ```
    /// `mem` is left as an `Option` so that the CPU can be extended
    /// to accept an initram after construction.
    pub fn new(mem: Option<Vec<Wrapping<u8>>>) -> Self {
        match mem {
            Some(mem) => Self {
                regs: Default::default(),
                mem,
            },
            None => {
                panic!("tcpuemu not provided init memory file.");
            }
        }
    }

    /// Run the CPU by repeatedly calling [`Instruction::execute`] and
    /// [`Instruction::execute`] and incrementing `r_p` in
    /// [`Cpu::regs`], like so:
    /// ```
    /// let ins = Instruction::new(self.mem[self.regs.r_p.0 as usize]);
    /// ins.execute(self);
    /// ```
    pub fn run(&mut self) {
        loop {
            self.dump_state();
            let ins = Instruction::new(self.mem[self.regs.r_p.0 as usize]);
            let prev_r_p = self.regs.r_p;
            self.regs.r_p += Wrapping(1);
            ins.execute(self);
            if prev_r_p == self.regs.r_p {
                println!("Detected forever loop, halting CPU.");
                return;
            }
        }
    }

    /// Print out the state of the CPU registers.
    fn dump_state(&self) {
        print!("A = {:02x}, ", self.regs.r_a);
        print!("B = {:02x}, ", self.regs.r_b);
        print!("M = {:02x}, ", self.regs.r_m);
        println!("P = {:02x}", self.regs.r_p);
    }
}

/// Contains the four registers of TinyCPU. These are intended to wrap
/// --- some assembly programs rely on this behavior.
#[derive(Default)]
struct Registers {
    r_a: Wrapping<u8>,
    r_b: Wrapping<u8>,
    r_m: Wrapping<u8>,
    r_p: Wrapping<u8>,
}

/// Implements the decoding and execution of each instruction opcode.
struct Instruction {
    opcode: Opcode,
    imm: Option<Wrapping<u8>>,
}

impl Instruction {
    /// Decode an instruction and for LDI, an immediate with the
    /// instruction. Acts as a constructor.
    pub fn new(encoded: Wrapping<u8>) -> Self {
        let opcode_enc = encoded >> 4;
        let opcode = match opcode_enc.0 {
            0x0 => Opcode::AND,
            0x1 => Opcode::OR,
            0x2 => Opcode::INV,
            0x3 => Opcode::ADD,
            0x4 => Opcode::LDI,
            0x5 => Opcode::LDM,
            0x6 => Opcode::STM,
            0x8 => Opcode::SWAB,
            0x9 => Opcode::SWMB,
            0xA => Opcode::CPPA,
            0xB => Opcode::CPAM,
            0xC => Opcode::JU,
            0xD => Opcode::JE,
            0xE => Opcode::JL,
            0xF => Opcode::JG,
            _ => panic!("Invalid opcode encoding {encoded}"),
        };

        let imm = match opcode {
            Opcode::LDI => Some(encoded & Wrapping(0xf)),
            _ => None,
        };

        Self { opcode, imm }
    }

    /// Execute an instruction and update the CPU state.
    pub fn execute(&self, cpu: &mut Cpu) {
        match self.opcode {
            Opcode::AND => cpu.regs.r_a &= cpu.regs.r_b,
            Opcode::OR => cpu.regs.r_a |= cpu.regs.r_b,
            Opcode::INV => cpu.regs.r_a = !cpu.regs.r_a,
            Opcode::ADD => cpu.regs.r_a += cpu.regs.r_b,
            Opcode::LDI => {
                cpu.regs.r_a &= Wrapping(0xf);
                cpu.regs.r_a <<= 4;
                cpu.regs.r_a |= self.imm.expect("LDI requires an immediate.");
            }
            Opcode::LDM => cpu.regs.r_a = cpu.mem[cpu.regs.r_m.0 as usize],
            Opcode::STM => cpu.mem[cpu.regs.r_m.0 as usize] = cpu.regs.r_a,
            Opcode::SWAB => (cpu.regs.r_a, cpu.regs.r_b) = (cpu.regs.r_b, cpu.regs.r_a),
            Opcode::SWMB => (cpu.regs.r_m, cpu.regs.r_b) = (cpu.regs.r_b, cpu.regs.r_m),
            Opcode::CPPA => cpu.regs.r_a = cpu.regs.r_p,
            Opcode::CPAM => cpu.regs.r_m = cpu.regs.r_a,
            Opcode::JU => cpu.regs.r_p = cpu.regs.r_m,
            Opcode::JE => {
                if cpu.regs.r_a == cpu.regs.r_b {
                    cpu.regs.r_p = cpu.regs.r_m
                }
            }
            Opcode::JL => {
                if cpu.regs.r_a < cpu.regs.r_b {
                    cpu.regs.r_p = cpu.regs.r_m
                }
            }
            Opcode::JG => {
                if cpu.regs.r_a > cpu.regs.r_b {
                    cpu.regs.r_p = cpu.regs.r_m
                }
            }
        }
    }
}

/// Opcode for each instruction type.
enum Opcode {
    AND,
    OR,
    INV,
    ADD,
    LDI,
    LDM,
    STM,
    SWAB,
    SWMB,
    CPPA,
    CPAM,
    JU,
    JE,
    JL,
    JG,
}
