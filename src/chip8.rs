use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
} 

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct CPU {
    pc: usize,
    index_register: usize,
    stack: Vec<usize>,
    memory: [u8; 4096],  
    display: Vec<u8>,
    v_register: [u8; 16],
    a_x: u16,
    a_y: u16,
    a_n: u16,
    a_nn: u16,
    a_nnn: u16,
    key_map: u16,
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl CPU {
    pub fn display_matrix(&self) -> *const u8 {
        self.display.as_ptr()
    }

    pub fn set_key_map(&mut self, map: u16) {
        self.key_map = map; 
    }

    pub fn is_pressed(&self, mask: u8) -> u16 {
        console::log_1(&format!("Check if pressed {} {}", self.key_map, mask).into());
        self.key_map & u16::pow(2, mask.into()) 
    } 

    pub fn new() -> CPU {

        let display: Vec<u8> = (0..64*32)
            .map(|_| { 0})
            .collect();
        CPU {
            pc: 512,
            index_register: 0,
            memory: [0; 4096],
            stack: vec![],
            v_register: [0;16],
            display,
            a_x: 0,
            a_y: 0,
            a_n: 0,
            a_nn: 0,
            a_nnn: 0,
            key_map: 0,
            delay_timer: 0,
            sound_timer: 0,
        }

    }

    pub fn load_program_at(&mut self, start: usize, program: Box<[u8]>) {
        let vec = program.into_vec();
        for (index, op) in vec.iter().enumerate() {
            self.memory[start + index] = *op;
        }    
    }

    pub fn tick(&mut self, delta: u8) {
        if self.delay_timer < delta { self.delay_timer = 0 }
        else { self.delay_timer -= delta }
        if self.sound_timer < delta { self.sound_timer = 0 }
        else { self.sound_timer -= delta }
        // let _timer = Timer::new("CPU::tick");
        /* const hb = mem.read(pc);
        const lb = mem.read(pc+1);
        const opcode = (hb << 8) | lb
        X = hb & 0x0f;
        N = lb & 0x0f;
        Y = (lb >> 4) & 0x0f;
        NN = lb;
        NNN = (X << 8) | lb;
        //console.log(hb, lb, opcode.toString(16));
        pc += 2;
        switch (opcode) {
        case 0x00e0: clear00e0();
        break;
        case (0x1000) | NNN: jump1nnn();
        break;
        case (0x6000) | (X << 8) | NN: set6xnn();
        break;
        case (0x7000) | (X << 8) | NN: add7xnn();
        break;
        case (0xA000) | NNN: setannn();
        break;
        case (0xD000) | (X << 8) | (Y << 4) | N: displaydxyn();
        break;
        default:
        //console.log("Miss", X.toString(16), Y.toString(16), N.toString(16))
        }
        */
        let hb: u16 = self.memory[self.pc].into();
        let lb: u16 = self.memory[self.pc + 1].into();
        let opcode: u16 = ((hb << 8) | lb).into();
        self.a_x = hb & 0x0f;
        self.a_n = lb & 0x0f;
        self.a_y = (lb >> 4) & 0x0f;
        self.a_nn = lb;
        self.a_nnn = ((self.a_x << 8) | lb).into();
        self.pc += 2;
        self.process(opcode);
    }

    fn process(&mut self, opcode: u16) {
        console::log_1(&format!("OPCODE: {:#06x} pc: {}", opcode, self.pc).into());
        if opcode == 0x00e0 { self.clear_screen() }
        else if opcode == 0x1000 | self.a_nnn { self.jump() }
        else if opcode == 0x2000 | self.a_nnn { self.start_subroutine() }
        else if opcode == 0x00ee { self.end_subroutine() }
        else if opcode == 0x3000 | self.a_x << 8 | self.a_nn { self.skip_3() }
        else if opcode == 0x4000 | self.a_x << 8 | self.a_nn { self.skip_4() }
        else if opcode == 0x5000 | self.a_x << 8 | self.a_y << 4 { self.skip_5() }
        else if opcode == 0x9000 | self.a_x << 8 | self.a_y << 4 { self.skip_9() }
        else if opcode == 0x6000 | self.a_x << 8 | self.a_nn { self.set_6() }
        else if opcode == 0x7000 | self.a_x << 8 | self.a_nn { self.add_7() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 { self.set_8() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x01 { self.binary_or() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x02 { self.binary_and() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x03 { self.logical_xor() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x04 { self.add_8() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x05 { self.subtract_85() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x07 { self.subtract_87() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x06 { self.shift_86() }
        else if opcode == 0x8000 | self.a_x << 8 | self.a_y << 4 | 0x0E { self.shift_8e() }
        else if opcode == (0xA000 | self.a_nnn) { self.set_index() }
        else if opcode == 0xB000 | self.a_nnn { self.jump_with_offset() }
        else if opcode == 0xC000 | self.a_x << 8 | self.a_nn { self.random() }
        else if opcode == 0xD000 | self.a_x << 8 | self.a_y << 4 |self.a_n { self.display() }
        else if opcode == 0xE09E | self.a_x << 8 { self.skip_key_9e() }
        else if opcode == 0xE0A1 | self.a_x << 8 { self.skip_key_a1() }
        else if opcode == 0xF007 | self.a_x << 8 { self.set_vx_from_timer() }
        else if opcode == 0xF015 | self.a_x << 8 { self.set_timer_from_vx() }
        else if opcode == 0xF018 | self.a_x << 8 { self.set_sound_timer_from_vx() }
        else if opcode == 0xF01E | self.a_x << 8 { self.add_to_index() }
        else if opcode == 0xF00A | self.a_x << 8 { self.get_key() }
        else if opcode == 0xF029 | self.a_x << 8 { self.font_character() }
        else if opcode == 0xF033 | self.a_x << 8 { self.binary_coded_decimal_conversion() }
        else if opcode == 0xF055 | self.a_x << 8 { self.store_memory() }
        else if opcode == 0xF065 | self.a_x << 8 { self.load_memory() }
        else {
            console::log_1(&format!("opcode missed: {:#06x} {} {}", opcode, 0xA000 | self.a_nnn, self.a_nnn ).into());
        }
    }

    fn clear_screen(&mut self) {
        let _timer = Timer::new("CPU::clear_screen");
        console::log_1(&format!("Clear screen").into());
        self.display = self.display.iter().map(|x| {0}).collect(); 
    }

    fn jump(&mut self) {
        console::log_1(&format!("Jump from {:#06x} to {:#06x}", self.pc, self.a_nnn).into());
        //console::log_1(&format!("JUMP from {} to {}", self.pc, self.a_nnn).into());
        self.pc = self.a_nnn as usize;   
    }


    fn start_subroutine(&mut self) {
        console::log_1(&format!("start_subroutine, pc from {} to {}", self.pc, self.a_nnn).into());
        self.stack.push(self.pc);
        self.pc = self.a_nnn as usize;   
    }

    fn end_subroutine(&mut self) {
        match self.stack.pop() {
            Some(x) => self.pc = x,
            None => console::error_1(&"Empty Stack: Cannot end subroutine.".into())
        }
        console::log_1(&format!("end_subroutine pc back to {}", self.pc).into());
    }

    fn skip_3(&mut self) {
        console::log_1(&format!("skip_3").into());
        if u16::from(self.v_register[self.a_x as usize]) != self.a_nn {
            return;
        } self.pc += 2
    }

    fn skip_4(&mut self) {
        console::log_1(&format!("skip_4").into());
        if u16::from(self.v_register[self.a_x as usize]) == self.a_nn {
            return;
        } self.pc += 2
    }

    fn skip_5(&mut self) {
        console::log_1(&format!("skip_5").into());
        if self.v_register[self.a_x as usize] != self.v_register[self.a_y as usize] {
            return;
        } self.pc += 2
    }

    pub fn skip_9(&mut self) {
        console::log_1(&format!("skip_9").into());
        if self.v_register[self.a_x as usize] == self.v_register[self.a_y as usize] {
            return;
        } self.pc += 2
    }

    pub fn set_6(&mut self) {
        console::log_1(&format!("set_6 set v[{}] to {}", self.a_x, self.a_nn).into());
        self.v_register[self.a_x as usize] = self.a_nn as u8; 
        //console::log_1(&format!("set vx {} {} {}", self.a_x, self.a_y, self.a_nnn).into());
    }

    pub fn add_7(&mut self) {
        console::log_1(&format!("add_7").into());
        self.v_register[self.a_x as usize] += self.a_nn as u8; 
    }

    pub fn set_8(&mut self) {
        console::log_1(&format!("set_8 v[{}] to {}", self.a_x, self.get_vy()).into());
        self.v_register[self.a_x as usize] = self.v_register[self.a_y as usize]
    }

    pub fn binary_or(&mut self) {
        console::log_1(&format!("binary_or").into());
        self.v_register[self.a_x as usize] |= self.v_register[self.a_y as usize]
    }

    pub fn binary_and(&mut self) {
        console::log_1(&format!("binary_and").into());
        self.v_register[self.a_x as usize] &= self.v_register[self.a_y as usize]
    }

    pub fn logical_xor(&mut self) {
        console::log_1(&format!("logical_xor").into());
        self.v_register[self.a_x as usize] ^= self.v_register[self.a_y as usize]
    }

    pub fn add_8(&mut self) {
        console::log_1(&format!("add_8").into());
        if (self.v_register[self.a_x as usize] as u16 + self.v_register[self.a_y as usize] as u16)  > 255 {
            self.v_register[15] = 1;
        } else {
            self.v_register[15] = 0;
        }
        self.v_register[self.a_x as usize] += self.v_register[self.a_y as usize]
    }

    pub fn subtract_85(&mut self) {
        console::log_1(&format!("subtract_85").into());
        if self.v_register[self.a_x as usize] < self.v_register[self.a_y as usize] { self.v_register[15] = 0 }
        else { self.v_register[15] = 1 }
        self.v_register[self.a_x as usize] -= self.v_register[self.a_y as usize];
    }

    pub fn subtract_87(&mut self) {
        console::log_1(&format!("subtract_87").into());
        if self.v_register[self.a_y as usize] < self.v_register[self.a_x as usize] { self.v_register[15] = 0 }
        else { self.v_register[15] = 1 }
        self.v_register[self.a_x as usize] = self.v_register[self.a_y as usize] - self.v_register[self.a_x as usize] ;
    }

    // AMBIGOUS INSTRUCTION
    pub fn shift_86(&mut self) {
        console::log_1(&format!("shift_86").into());
        self.set_vx(self.get_vy());

        if self.get_vx() & 0b00000001 > 0 { self.v_register[15] = 1}
        else { self.v_register[15] = 0 }

        self.set_vx(self.get_vx() >> 1);
    }

    fn get_vx(&self) -> u8 {
        self.v_register[self.a_x as usize] 
    }

    fn set_vx(&mut self, value: u8) {
        self.v_register[self.a_x as usize] = value;
    }

    fn get_vy(&self) -> u8 {
        self.v_register[self.a_y as usize] 
    }

    fn set_vy(&mut self, value: u8) {
        self.v_register[self.a_y as usize] = value;
    }

    pub fn shift_8e(&mut self) {
        console::log_1(&format!("shift_8e").into());
        self.set_vx(self.get_vy());

        if self.get_vx() & 0b10000000 > 0 { self.v_register[15] = 1}
        else { self.v_register[15] = 0 }

        self.set_vx(self.get_vx() << 1);
    }

    pub fn set_index(&mut self) {
        console::log_1(&format!("set_index to {:#06x}", self.a_nnn).into());
        //console::log_1(&format!("set index to {}", self.a_nnn).into());
        self.index_register = self.a_nnn as usize; 
    }

    // AMBIGOUS INSTRUCTION
    pub fn jump_with_offset(&mut self) {
        console::log_1(&format!("jump_with_offset").into());
        self.pc = (self.a_nnn as usize) + self.v_register[0] as usize; 
    }

    pub fn random(&mut self) {
        let mut buf = [0u8; 32];
        getrandom::getrandom(&mut buf);
        let n = buf[0];
        console::log_1(&format!("random: {} set to v[{}]", n & self.a_nn as u8, self.a_x).into());
        self.set_vx(n & self.a_nn as u8);
    }

    /* //console.log("Display", X, Y, N.toString(16));
    let x = vRegisters[X] % 64;
    let y = vRegisters[Y] % 32;
    //console.log("Display", x, y, N.toString(16));

    vRegisters[flagRegisterI] = 0;
    for(let row = 0; row < N; row++) {
    // //console.log("Reading", indexRegister + row);
    const data = mem.read(indexRegister + row);
    for (let col = 0; col < 8 && col + x < 64; col++) {
    let shouldON = !!(data & (1 << (7 - col)));
    let displayIndex = 64*(row + y) + (col + x);
    let isOn = display[displayIndex];
    if (isOn && shouldON) {
    vRegisters[flagRegisterI] = 1; 
    display[displayIndex] = false;
    } else if (shouldON && !isOn) {
    display[displayIndex] = true;
    }
    }
    if (y + row >= 32) break;
    }
    }
*/
    pub fn display(&mut self) {

        console::log_1(&format!("display").into());
        let _timer = Timer::new("CPU::display");
        let x = (self.v_register[self.a_x as usize] % 64) as u16;
        let y = (self.v_register[self.a_y as usize] % 32) as u16;
        self.v_register[15] = 0;

        for row in 0..self.a_n {
            for col in 0..8 {
                let data = self.memory[(self.index_register + row as usize)];
                let should_on = (data & (1 << (7 - col))) != 0;
                if col + x < 64 && row + y < 32 {
                    //console::log_1(&format!("data {}, row {}, col {}", self.index_register, x, y).into());
                    let display_index = 64*(row + y as u16) + (col + x) as u16;
                    let is_on = self.display[display_index as usize] == 1;

                    if is_on && should_on {
                        self.v_register[15] = 1;
                        self.display[display_index as usize] = 0;
                    } else if should_on && !is_on {
                        //console::log_1(&format!("TURN ON {} {}", row + y, col + x).into());
                        self.display[display_index as usize] = 1;
                    }
                }
            }
            if y as u16 + row >= 32 {break};
        }
    }

    pub fn skip_key_9e(&mut self) {
        console::log_1(&format!("skip_key_9e").into());
        if self.is_pressed(self.get_vx()) > 0 { self.pc += 2 }
    }

    pub fn skip_key_a1(&mut self) {
        console::log_1(&format!("skip_key_a1").into());
        if self.is_pressed(self.get_vx()) == 0 { self.pc += 2 }
    }

    pub fn set_vx_from_timer(&mut self) {
        console::log_1(&format!("set_vx_from_timer").into());
        self.set_vx(self.delay_timer)
    }

    pub fn set_timer_from_vx(&mut self) {
        console::log_1(&format!("set_timer_from_vx").into());
        self.delay_timer = self.get_vx()
    }

    pub fn set_sound_timer_from_vx(&mut self) {
        console::log_1(&format!("set_sound_timer_from_vx").into());
        self.sound_timer = self.get_vx()
    }

    pub fn add_to_index(&mut self) {
        console::log_1(&format!("add_to_index from v[{}] {}", self.a_x, self.get_vx()).into());
        if self.index_register + self.get_vx() as usize > 0xFFF { self.v_register[15] = 1 } 
        else { self.v_register[15] = 0 }

        self.index_register += self.get_vx() as usize
    }

    pub fn get_key(&mut self) {
        console::log_1(&format!("get_key").into());
        if self.key_map == 0 { self.pc -= 2 }
        else {
            for i in 0..16 {
                if self.is_pressed(i) > 0 {
                    self.set_vx(i);
                    break;
                }
            }
        }
    }

    pub fn font_character(&mut self) {
        console::log_1(&format!("Request font {}", self.get_vx()).into());
        if self.get_vx() < 0x10 {
            self.index_register = 0x050 + (self.get_vx() * 5) as usize;
        } else {
            console::log_1(&format!("Invalid font character requested {}", self.get_vx()).into())
        }
        console::log_1(&format!("Request font {} {:#06x}", self.get_vx(), self.index_register).into());
    }

    pub fn binary_coded_decimal_conversion(&mut self) {
        console::log_1(&format!("binary_coded_decimal_conversion").into());
        self.memory[self.index_register] = (self.get_vx() / 100) % 10;
        self.memory[self.index_register + 1] = (self.get_vx() / 10) % 10;
        self.memory[self.index_register + 2] = self.get_vx() % 10;
    }

    pub fn store_memory(&mut self) {
        console::log_1(&format!("store_memory from v[0] to v[{}]", self.get_vx()).into());
        for i in 0..(self.get_vx() + 1) {
            if i > 15 {break;}
            self.memory[self.index_register + i as usize] = self.v_register[i as usize];
        }
    }

    pub fn load_memory(&mut self) {
        console::log_1(&format!("load_memory").into());
        for i in 0..(self.get_vx() + 1) {
            if i > 15 {break;}
            self.v_register[i as usize] = self.memory[self.index_register + i as usize];
        }
    }
}
