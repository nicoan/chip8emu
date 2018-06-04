use std::fs::File;
use std::io::Read;
use std::io::Error;
use rand::random;
use termion::{cursor, clear};
use std::io::{Write, stdout, stdin};
use std::{thread, time};

// VF
const FLAG_REGISTER: usize = 15;

const fontset: [u8; 80] =
[
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub enum MachineState {
    SuccessfulExecution,
    Draw,
    WaitForKeyboard(u8),
}

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

    // Keypad
    keypad: u16,

    // Timers
    delay_timer: u8,
    sound_timer: u8
}

impl State {
    pub fn new(filename: String) -> Result<State, Error> {
        let mut memory: [u8; 4096] = [0x0; 4096];

        // Load the fontset
        for i in 0..80 {
            memory[0x50 + i] = fontset[i];
        }

        // Read CHIP-8 File
        // Allocate the rom in memory
        let mut file = try!(File::open(filename));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
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
            screen: [[0x0; 8]; 32],
            keypad: 0x0,
            delay_timer: 0x0,
            sound_timer: 0x0,
        })
    }

    pub fn execute_instruction(&mut self) -> Result<MachineState, String> {
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
                Ok(MachineState::SuccessfulExecution)
            }

            // 00EE - RET
            // Return from a subroutine.
            // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp];
                self.sp -= 1;
                Ok(MachineState::SuccessfulExecution)
            }

            //1nnn - JP addr
            //Jump to location nnn.
            //The interpreter sets the program counter to nnn.
            (0x1, _, _, _) => {
                let  address: u16 = opcode & 0x0FFF;
                self.pc = address;
                Ok(MachineState::SuccessfulExecution)
            }

            // 2nnn - CALL addr
            // Call subroutine at nnn.
            // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                let  address: u16 = opcode & 0x0FFF;
                self.pc = address;
                Ok(MachineState::SuccessfulExecution)
            }

            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            (0x3, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                if self.registers[r as usize] == kk {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            (0x4, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                if self.registers[r as usize] != kk {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            (0x5, r1, r2, 0x0) => {
                if self.registers[r1 as usize] == self.registers[r2 as usize] {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            //6xkk - LD Vx, byte
            //Set Vx = kk.
            //The interpreter puts the value kk into register Vx.
            (0x6, x, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = kk;
                Ok(MachineState::SuccessfulExecution)
            }

            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the result in Vx.
            (0x7, r, _, _) => {
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.registers[r as usize] += kk;
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy0 - LD Vx, Vy
            // Set Vx = Vy.
            // Stores the value of register Vy in register Vx.
            (0x8, x, y, 0x0) => {
                self.registers[x as usize] = self.registers[y as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x1) => {
                self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy2 - AND Vx, Vy
            // Set Vx = Vx AND Vy.
            // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x2) => {
                self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy.
            // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
            (0x8, x, y, 0x3) => {
                self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry.
            // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1,
            // otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
            (0x8, x, y, 0x4) => {
                // TODO: Check the into, not sure how it works
                let result: u16 = (self.registers[x as usize] + self.registers[y as usize]).into();
                self.registers[x as usize] = result as u8;
                self.registers[FLAG_REGISTER] = if result > 255 { 1 } else { 0 };
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy5 - SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT borrow.
            // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
            (0x8, x, y, 0x5) => {
                self.registers[FLAG_REGISTER] = if self.registers[x as usize] > self.registers[y as usize] { 1 } else { 0 };
                self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy6 - SHR Vx {, Vy}
            // Set Vx = Vx SHR 1.
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            (0x8, x, y, 0x6) => {
                self.registers[FLAG_REGISTER] = if 0x1 & self.registers[x as usize] == 1 { 1 } else { 0 };
                self.registers[x as usize] = self.registers[x as usize] >> 1;
                Ok(MachineState::SuccessfulExecution)
            }

            // 8xy7 - SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow.
            // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
            (0x8, x, y, 0x7) => {
                self.registers[FLAG_REGISTER] = if self.registers[y as usize] > self.registers[x as usize] { 1 } else { 0 };
                self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            //8xyE - SHL Vx {, Vy}
            //Set Vx = Vx SHL 1.
            //If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            (0x8, x, y, 0xE) => {
                self.registers[FLAG_REGISTER] = if self.registers[x as usize] >> 3 == 1 { 1 } else { 0 };
                self.registers[x as usize] = (self.registers[x as usize] >> 1) | 0x8;
                Ok(MachineState::SuccessfulExecution)
            }

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            (0x9, x, y, 0x0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // Annn - MVI nnn
            // Load index register with constant xxx
            (0xA, _, _, _) => {
                self.index = opcode & 0x0FFF;
                Ok(MachineState::SuccessfulExecution)
            }

            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            (0xB, _, _, _) => {
                let address: u16 = opcode & 0x0FFF;
                self.pc = address + self.registers[0 as usize] as u16;
                Ok(MachineState::SuccessfulExecution)
            }

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk. The interpreter generates a random number
            // from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx
            (0xC, r, _, _) => {
                let number: u8 = (opcode & 0x00FF) as u8;
                let random_number = random::<u8>();
                self.registers[r as usize] = number & random_number;
                //println!("{} - {} - {}", random_number, number, self.registers[r as usize]);
                Ok(MachineState::SuccessfulExecution)
            }

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed
            // as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any
            // pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is
            // outside the coordinates of the display, it wraps around to the opposite side of the screen.
            // TODO Check if there is a collision
            (0xD, x, y, n) => {
                let v_x: u8 = self.registers[x as usize];
                let v_y: u8 = self.registers[y as usize];
                let screen_vertical_index: u8 = v_x / 8;
                let reminder: u8 = v_x % 8;
                for i in 0..n {
                    let sprite_part = self.memory[(self.index + i as u16) as usize];
                    if reminder != 0 {
                        // Draw left part of the sprite
                        let sprite_left = sprite_part >> reminder;
                        self.screen[(v_y + i) as usize][screen_vertical_index as usize] ^= sprite_left;

                        // Prevent screen overflow
                        if screen_vertical_index < 7 {
                            // Draw right part of the sprite
                            let sprite_right = (sprite_part << (8 - reminder));
                            self.screen[(v_y + i) as usize][(screen_vertical_index + 1) as usize] ^= sprite_right;
                        }
                    } else {
                        self.screen[(v_y + i) as usize][screen_vertical_index as usize] = sprite_part;
                    }
                }

                Ok(MachineState::Draw)
            }

            // TODO in Exxx opcodes check the bit displacement (if its ok)

            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
            // PC is increased by 2.
            (0xE, x, 0x9, 0xE) => {
                if self.keypad >> self.registers[x as usize] & 0x1 == 1 {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position,
            // PC is increased by 2.
            (0xE, x, 0xA, 0x1) => {
                if (self.keypad >> self.registers[x as usize] & 0x1 == 1) {
                    self.pc += 2;
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            // The value of DT is placed into Vx.
            (0xF, x, 0x0, 0x7) => {
                self.registers[x as usize] = self.delay_timer;
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx0A - LD Vx, K
            // Wait for a key press, store the value of the key in Vx.
            // All execution stops until a key is pressed, then the value of that key is stored in Vx.
            (0xF, x, 0x0, 0xA) => {
                Ok(MachineState::WaitForKeyboard(self.registers[x as usize]))
            }

            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            // DT is set equal to the value of Vx.
            (0xF, x, 0x1, 0x5) => {
                self.delay_timer = self.registers[x as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            // ST is set equal to the value of Vx.
            (0xF, x, 0x1, 0x8) => {
                self.sound_timer = self.registers[x as usize];
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx1E - ADD I, Vx
            // Set I = I + Vx.
            // The values of I and Vx are added, and the results are stored in I.
            (0xF, x, 0x1, 0xE) => {
                self.index += self.registers[x as usize] as u16;
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx29 - LD F, Vx
            // Set I = location of sprite for digit Vx.
            // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
            (0xF, x, 0x2, 0x9) => {
                // We multiply by 4 beacuse every digit sprite starts at a multiple of 4
                self.index = (self.registers[x as usize] * 4).into();
                Ok(MachineState::SuccessfulExecution)
            }

            //Fx33 - LD B, Vx
            //Store BCD representation of Vx in memory locations I, I+1, and I+2.
            //The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.


            //Fx55 - LD [I], Vx
            //Store registers V0 through Vx in memory starting at location I.
            //The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
            (0xF, x, 0x5, 0x5) => {
                let n: u8 = self.registers[x as usize];
                for i in 0..n {
                    let index: usize = (self.index + i as u16) as usize;
                    self.memory[index] = self.registers[i as usize];
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // Fx65 - LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I.
            // The interpreter reads values from memory starting at location I into registers V0 through Vx.
            (0xF, x, 0x6, 0x5) => {
                for i in 0..x {
                    self.registers[i as usize] = self.memory[(self.index + i as u16) as usize];
                }
                Ok(MachineState::SuccessfulExecution)
            }

            // Invalid opcodes
            _ => {
                let errorMessage: String = format!("Critical error: attempted to execute {:x} (invalid opcode)", opcode);
                Err(errorMessage)
            }
        }
    }

    pub fn wait_key_press(&mut self, key: u8) {
        println!("pressed {}", key);
        thread::sleep(time::Duration::from_millis(1000));
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

    fn print_screen2(&mut self) {
        for y in 0..32 {
            for x in 0..8 {
                for i in 0..8 {
                    if ((self.screen[y as usize][x as usize] << i) & 0x80) == 0x80 {
                        print!("██");
                    } else {
                        print!("  ");
                    }
                }
            }
            println!(" ")
        }
    }

    pub fn print_screen(&mut self) {
        println!("{}", clear::All);
        let stdout = stdout();
        let mut stdout = stdout.lock();
        for y in (0..16).map(|x| x * 2) {
            for x in 0..8 {
                for i in 0..8 {
                    let top_square: bool = (self.screen[y as usize][x as usize] << i) & 0x80 == 0x80;
                    let bottom_square: bool = (self.screen[y + 1 as usize][x as usize] << i) & 0x80 == 0x80;
                    let x_coord = (x * 8) + i + 2;
                    let y_coord: u16 = (y / 2) as u16 + 2;
                    match (top_square, bottom_square) {
                        (true, true) => write!(stdout, "█{}", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (true, false) => write!(stdout, "▀{}", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, true) => write!(stdout, "▄{}", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, false) => write!(stdout, " {}", cursor::Goto(x_coord, y_coord)).unwrap()
                    }
                }
            }
        }
        println!("");
        stdout.flush().unwrap();
    }
}


