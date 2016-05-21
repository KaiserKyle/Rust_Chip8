use std::io;

const CHIP8_FONTSET: [u8; 80] = [
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

#[derive(Default)]
#[derive(Debug)]
struct Chip8State {
    index: u16,
    pc: usize,
    delay_timer: u16,
    sound_timer: u16,
    stack_pointer: usize,
    stack: Vec<usize>,
    v: Vec<u16>
}

fn main() {
    // Initialize State
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    let mut gfx = vec![0u8; 2048];
    let mut key_press = vec![0u8; 16];
    
    // Load fontset
    for x in 0..80 {
        memory[x] = CHIP8_FONTSET[x];
    }
    
    // Load program
    state.pc = 0x200;
    state.sound_timer = 0xF;
    state.delay_timer = 0x10;
    state.stack = vec![0usize; 16];
    state.v = vec![0u16; 16];
    
    // Test data
    memory[0x200] = 0xA2;
    memory[0x201] = 0xF0;
    memory[0x202] = 0x22;
    memory[0x203] = 0x08;
    memory[0x208] = 0x31;
    memory[0x209] = 0x01;
    memory[0x20C] = 0x42;
    memory[0x20D] = 0x01;
    memory[0x210] = 0x12;
    memory[0x211] = 0x00;
    state.v[1] = 1;
    
    // Emulation loop
    loop {
        emulate_cycle(&mut state, &memory);
    }
    //println!("Opcode {}", opcode);
}

fn emulate_cycle(state: &mut Chip8State, memory: &Vec<u8>) {
    let mut line = String::new();

    let opcode = get_opcode(state.pc, &memory);
    println!("Opcode: {:X}", opcode);
    
    execute_opcode(opcode, state);
    update_timers(state);
    
    println!("{:?}", state);
    let _ = io::stdin().read_line(&mut line);
}

fn get_opcode(pc: usize, memory: &Vec<u8>) -> u16 {
    // Shift upper bits over 8 bits, then OR with lower 8 bits.
    u16::from(memory[pc]) << 8 | u16::from(memory[pc + 1])
}

fn execute_opcode(opcode: u16, state: &mut Chip8State) {
    let decode = opcode & 0xF000;
    match decode {
        0x1000 => {
            println!("0x1 opcode");
            state.pc = (opcode & 0x0FFF) as usize;
            }
        0x2000 => {
            println!("0x2 opcode");
            state.stack[state.stack_pointer] = state.pc;
            state.stack_pointer += 1;
            state.pc = (opcode & 0x0FFF) as usize;
            }
        0x3000 => {
            println!("0x3 opcode");
            if state.v[((opcode & 0x0F00) >> 8) as usize] == (opcode & 0x00FF) {
                state.pc += 2;
                }
            state.pc += 2;
            }
        0x4000 => {
            println!("0x4 opcode");
            if state.v[((opcode & 0x0F00) >> 8) as usize] != (opcode & 0x00FF) {
                state.pc += 2;
                }
            state.pc += 2;
            }
        0xA000 => {
            println!("0xA opcode");
            state.index = opcode & 0x0FFF;
            state.pc += 2;
            }
        _ => { println!("Unknown opcode: {:X}", decode)}
    }
}

fn update_timers(state: &mut Chip8State) {
    if state.delay_timer > 0 {
        state.delay_timer -= 1;
    }
    if state.sound_timer > 0 {
        state.sound_timer -= 1;
    }
}