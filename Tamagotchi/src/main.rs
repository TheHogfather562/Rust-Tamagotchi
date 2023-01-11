#[macro_use]
extern crate twelve_bit;

use twelve_bit::u12::*;

pub struct CPU {
    pub register_a: U12,
    pub register_b: U12,
    pub program_counter: U12,
    pub flag_register: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: u12![0],
            register_b: u12![0],
            program_counter: u12![0], // Load step 0 and page 1
            flag_register: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<U12>) {
        self.program_counter = u12![0];

        loop {
            let pc :u32 = self.program_counter.into();
            let pc_8 :usize = (pc & 0b11111111).try_into().unwrap();
            let mem :u32 = program[pc_8].into();
            self.program_counter = self.program_counter.overflowing_add(u12![1]).0;

            let opcode :u16 = (mem >> 4).try_into().unwrap();
            let value :u8 = (mem & 0b1111).try_into().unwrap();

            match opcode{
                0b11110100 => {
                    self.flag_register |= 1 << (value - 1);
                }

                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}


fn main() {
    let mut cpu = CPU::new();
        cpu.interpret(vec![u12![0b111101000001], u12![0]]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flag_set_1() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![u12![0b111101000001], u12![0]]);
        assert_eq!(cpu.flag_register, 1);
    }
}
