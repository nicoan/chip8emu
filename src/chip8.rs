use std::fs::File;
use std::io::Read;
use std::io::Error;
use rand::random;

// VF
const FLAG_REGISTER: usize = 15;

// Represnts CHIP-8 current state
pub struct State {
    // Current opcode
    opcode: u16,

    // Main memory
    memory: [u8; 4096],

    // CPU Registers
    registers: [u8; 16],

    // Call Stack
    stack: [u16; 16],

    // Index register
    index: u16,

    // Program counter
    pc: u16,

    // Stack pointer
    sp: usize,

    // Screen bitmap
    screen: [[u8; 8]; 32], // 32 rows of 8 u8 (64bits)

    // Timers
    delay_timer: u8,
    sound_timer: u8
}

impl State {
    pub fn new(filename: String) -> Result<State, Error> {
        // Read CHIP-8 File
        let mut file = try!(File::open(filename));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));

        // Allocate it in memory
        let mut memory: [u8; 4096] = [0x0; 4096];
        for i in 0..buffer.len() {
            memory[0x200 + i] = buffer[i];
        }

        Ok(State {
            opcode: 0x0,
            pc: 0x200,
            index: 0x0,
            sp: 0x0,
            registers: [0x0; 16],
            memory: memory,
            stack: [0x0; 16],
            delay_timer: 0x0,
            sound_timer: 0x0,
            screen: [[0x0; 8]; 32]
        })
    }

    pub fn execute_instruction(&mut self) -> Result<(), String> {
        let opcode: u16 = try!(self.get_opcode());
        self.pc += 2;
        //let opcode_hex: String = format!("{:x}", opcode);
        //println!("{}", opcode_hex);
        //println!("{:?}", self.break_opcode(opcode));

        match self.break_opcode(opcode) {
            // 00E0 - CLS
            // Clear the display.
            (0x0, 0x0, 0xE, 0x0) => {
                self.screen = [[0x0; 8]; 32];
                Ok(())
            }

            //1nnn - JP addr
            //Jump to location nnn.
            //The interpreter sets the program counter to nnn.
            (0x1, _, _, _) => {
                let  address: u16 = opcode & 0x0FFF;
                self.pc = address;
                Ok(())
            }

            // 2nnn - CALL addr
            // Call subroutine at nnn.
            // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                let  address: u16 = opcode & 0x0FFF;
                self.pc = address;
                Ok(())
            }

            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            (0x3, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                if self.registers[r as usize] == kk {
                    self.pc += 2;
                }
                Ok(())
            },

            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            (0x4, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                if self.registers[r as usize] != kk {
                    self.pc += 2;
                }
                Ok(())
            },

            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            (0x5, r1, r2, 0x0) => {
                if self.registers[r1 as usize] == self.registers[r2 as usize] {
                    self.pc += 2;
                }
                Ok(())
            },

            //6xkk - LD Vx, byte
            //Set Vx = kk.
            //The interpreter puts the value kk into register Vx.
            (0x6, x, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = kk;
                Ok(())
            },

            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the result in Vx.
            (0x7, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.registers[r as usize] += kk;
                Ok(())
            },

            // 8xy0 - LD Vx, Vy
            // Set Vx = Vy.
            // Stores the value of register Vy in register Vx.
            (0x8, x, y, 0x0) => {
                self.registers[x as usize] = self.registers[y as usize];
                Ok(())
            },

            // 8xy1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x1) => {
                self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
                Ok(())
            },

            // 8xy2 - AND Vx, Vy
            // Set Vx = Vx AND Vy.
            // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x2) => {
                self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
                Ok(())
            },

            // 8xy3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy.
            // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x3) => {
                self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
                Ok(())
            },

            // 8xy4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry.
            // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1,
            // otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
            (0x8, x, y, 0x4) => {
                // TODO: Check the into, not sure how it works
                let result: u16 = (self.registers[x as usize] + self.registers[y as usize]).into();
                self.registers[x as usize] = result as u8;
                self.registers[FLAG_REGISTER] = if result > 255 { 1 } else { 0 };
                Ok(())
            },

            // 8xy5 - SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT borrow.
            // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
            (0x8, x, y, 0x5) => {
                self.registers[FLAG_REGISTER] = if self.registers[x as usize] > self.registers[y as usize] { 1 } else { 0 };
                self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
                Ok(())
            },

            // 8xy6 - SHR Vx {, Vy}
            // Set Vx = Vx SHR 1.
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            (0x8, x, y, 0x6) => {
                self.registers[FLAG_REGISTER] = if 0x1 & self.registers[x as usize] == 1 { 1 } else { 0 };
                self.registers[x as usize] = self.registers[x as usize] >> 1;
                Ok(())
            },

            // 8xy7 - SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow.
            // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
            (0x8, x, y, 0x7) => {
                self.registers[FLAG_REGISTER] = if self.registers[y as usize] > self.registers[x as usize] { 1 } else { 0 };
                self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
                Ok(())
            },

            //8xyE - SHL Vx {, Vy}
            //Set Vx = Vx SHL 1.
            //If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            (0x8, x, y, 0xE) => {
                self.registers[FLAG_REGISTER] = if self.registers[x as usize] >> 3 == 1 { 1 } else { 0 };
                self.registers[x as usize] = (self.registers[x as usize] >> 1) | 0x8;
                Ok(())
            },

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            (0x9, x, y, 0x0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
                Ok(())
            },

            // Annn - MVI nnn
            // Load index register with constant xxx
            (0xA, _, _, _) => {
                self.index = opcode & 0x0FFF;
                Ok(())
            },

            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            (0xB, _, _, _) => {
                let address: u16 = opcode & 0x0FFF;
                self.pc = address + self.registers[0 as usize] as u16;
                Ok(())
            },

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk. The interpreter generates a random number
            // from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx
            (0xC, r, _, _) => {
                let number: u8 = (opcode & 0x00FF) as u8;
                let random_number = random::<u8>();
                self.registers[r as usize] = number & random_number;
                //println!("{} - {} - {}", random_number, number, self.registers[r as usize]);
                Ok(())
            },

            (0xD, x, y, n) => {
                let mut i: u8 = 0;
                let v_x: u8 = self.registers[x as usize];
                let v_y: u8 = self.registers[y as usize];
                let screen_vertical_index: u8 = v_x / 8;
                let reminder: u8 = v_x % 8;
                println!("{} {} {} {}", v_x, v_y, screen_vertical_index, reminder);
                for p in 0..n {
                    let sprite_part = self.memory[(self.index + i as u16) as usize];
                    if reminder != 0 {
                        // Draw left part of the sprite
                        let sprite_left = sprite_part >> reminder;
                        self.screen[(v_y + i) as usize][screen_vertical_index as usize] = self.memory[(self.index + i as u16) as usize] ;

                        // Prevent screen overflow
                        if screen_vertical_index < 7 {
                            // Draw right part of the sprite
                            let sprite_right = sprite_part << reminder;
                            self.screen[(v_y + i) as usize][(screen_vertical_index + 1) as usize] = self.memory[(self.index + i as u16) as usize] ;
                        }
                    } else {
                        self.screen[(v_y + i) as usize][screen_vertical_index as usize] = sprite_part;
                    }
                    i += 1;
                }

                self.print_screen();
                Ok(())
            },


            // Invalid opcodes
            _ => {
                println!("Invalid opcode");
                Err("Invalid opcode".to_string())
            }
        }
    }

    fn get_opcode(&mut self) -> Result<u16, String> {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 0x8 | (self.memory[(self.pc + 1) as usize] as u16);
        Ok (opcode)
    }

    fn break_opcode(&mut self, opcode: u16) -> (u8, u8, u8, u8) {
        ((opcode >> 12 & 0xF) as u8,
         (opcode >> 8 & 0xF) as u8,
         (opcode >> 4 & 0xF) as u8,
         (opcode & 0xF) as u8)
    }

    fn print_screen(&mut self) {
        for y in 0..32 {
            for x in 0..8 {
                for i in 0..8 {
                    if (self.screen[y as usize][x as usize] >> i & 0x1) == 1 {
                        print!("█");
                    } else {
                        print!(" ");
                    }
                }
            }
            println!(" ")
        }
    }
}


