#[macro_use]
extern crate glium;
extern crate schedule_recv;
extern crate time;
extern crate rand;

use std::io;
use std::io::prelude::*;

use std::fs::File;
use std::borrow::Cow;

use rand::Rng;

use glium::{DisplayBuild, Surface};
use glium::glutin;
use glium::index::PrimitiveType;

#[cfg(test)]
mod tests;

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
    v: Vec<u16>,
    gfx: Vec<u8>,
    key_press: Vec<u8>
}

fn main() {
    // Initialize State
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    
    // Load fontset
    for x in 0..80 {
        memory[x] = CHIP8_FONTSET[x];
    }
    
    // Open window
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .build_glium()
        .unwrap();
        
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.finish().unwrap();
    
    // Load program
    state.pc = 0x200;
    state.stack = vec![0usize; 16];
    state.v = vec![0u16; 16];
    state.gfx = vec![0u8; 2048];
    state.key_press = vec![0u8; 16];
    
    let mut file_data = Vec::new();
    let mut f = File::open("c:\\emu\\chip8\\BRIX.").unwrap();
    
    let file_size = f.read_to_end(&mut file_data).unwrap();
    
    for x in 0..file_size {
        memory[0x200 + x] = file_data[x];
    }
    
    // Test data
    /*memory[0x200] = 0xA2;
    memory[0x201] = 0xF0;
    memory[0x202] = 0x61;
    memory[0x203] = 0x01;
    memory[0x204] = 0x63;
    memory[0x205] = 0x01;
    memory[0x206] = 0x74;
    memory[0x207] = 0xA3;
    memory[0x208] = 0x23;
    memory[0x209] = 0x08;
    memory[0x308] = 0x31;
    memory[0x309] = 0x01;
    memory[0x30C] = 0x42;
    memory[0x30D] = 0x01;
    memory[0x310] = 0x51;
    memory[0x311] = 0x30;
    memory[0x314] = 0x12;
    memory[0x315] = 0x00;*/
    
    // Emulation loop
    // 60 Hz
    let tick = schedule_recv::periodic_ms(1000 / 60);
    loop {
        handle_keyboard(&mut state, &display);
        emulate_cycle(&mut state, &mut memory, &display);
        //tick.recv().unwrap();
    }
    //println!("Opcode {}", opcode);
}

fn handle_keyboard(state: &mut Chip8State, display: &glium::backend::glutin_backend::GlutinFacade) {
    for ev in display.poll_events() {
        match ev {
            glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(key)) => {
                match key {
                    glium::glutin::VirtualKeyCode::Key1 => {
                       state.key_press[0x1] = 1;
                    }
                    glium::glutin::VirtualKeyCode::Key2 => {
                       state.key_press[0x2] = 1;
                    }
                    glium::glutin::VirtualKeyCode::Key3 => {
                       state.key_press[0x3] = 1;
                    }
                    glium::glutin::VirtualKeyCode::Key4 => {
                       state.key_press[0xC] = 1;
                    }
                    glium::glutin::VirtualKeyCode::Q => {
                       state.key_press[0x4] = 1;
                    }
                    glium::glutin::VirtualKeyCode::W => {
                       state.key_press[0x5] = 1;
                    }
                    glium::glutin::VirtualKeyCode::E => {
                       state.key_press[0x6] = 1;
                    }
                    glium::glutin::VirtualKeyCode::R => {
                       state.key_press[0xD] = 1;
                    }
                    glium::glutin::VirtualKeyCode::A => {
                       state.key_press[0x7] = 1;
                    }
                    glium::glutin::VirtualKeyCode::S => {
                       state.key_press[0x8] = 1;
                    }
                    glium::glutin::VirtualKeyCode::D => {
                       state.key_press[0x9] = 1;
                    }
                    glium::glutin::VirtualKeyCode::F => {
                       state.key_press[0xE] = 1;
                    }
                    glium::glutin::VirtualKeyCode::Z => {
                       state.key_press[0xA] = 1;
                    }
                    glium::glutin::VirtualKeyCode::X => {
                       state.key_press[0x0] = 1;
                    }
                    glium::glutin::VirtualKeyCode::C => {
                       state.key_press[0xB] = 1;
                    }
                    glium::glutin::VirtualKeyCode::V => {
                       state.key_press[0xF] = 1;
                    }
                    _ => ()
            }
        }
            glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(key)) => {
                match key {
                    glium::glutin::VirtualKeyCode::Key1 => {
                        state.key_press[0x1] = 0;
                    }
                    glium::glutin::VirtualKeyCode::Key2 => {
                       state.key_press[0x2] = 0;
                    }
                    glium::glutin::VirtualKeyCode::Key3 => {
                       state.key_press[0x3] = 0;
                    }
                    glium::glutin::VirtualKeyCode::Key4 => {
                       state.key_press[0xC] = 0;
                    }
                    glium::glutin::VirtualKeyCode::Q => {
                       state.key_press[0x4] = 0;
                    }
                    glium::glutin::VirtualKeyCode::W => {
                       state.key_press[0x5] = 0;
                    }
                    glium::glutin::VirtualKeyCode::E => {
                       state.key_press[0x6] = 0;
                    }
                    glium::glutin::VirtualKeyCode::R => {
                       state.key_press[0xD] = 0;
                    }
                    glium::glutin::VirtualKeyCode::A => {
                       state.key_press[0x7] = 0;
                    }
                    glium::glutin::VirtualKeyCode::S => {
                       state.key_press[0x8] = 0;
                    }
                    glium::glutin::VirtualKeyCode::D => {
                       state.key_press[0x9] = 0;
                    }
                    glium::glutin::VirtualKeyCode::F => {
                       state.key_press[0xE] = 0;
                    }
                    glium::glutin::VirtualKeyCode::Z => {
                       state.key_press[0xA] = 0;
                    }
                    glium::glutin::VirtualKeyCode::X => {
                       state.key_press[0x0] = 0;
                    }
                    glium::glutin::VirtualKeyCode::C => {
                       state.key_press[0xB] = 0;
                    }
                    glium::glutin::VirtualKeyCode::V => {
                       state.key_press[0xF] = 0;
                    }
                    _ => ()
                }
            }
             _ => ()
        }
    }
    //println!("{:?}", state.key_press);
}

fn emulate_cycle(state: &mut Chip8State, memory: &mut Vec<u8>, display: &glium::backend::glutin_backend::GlutinFacade) {   
    let opcode = get_opcode(state.pc, &memory);
    //println!("Opcode: {:X}", opcode);
    
    let draw_flag = execute_opcode(opcode, state, memory);
    if draw_flag == true {
        //println!("Redraw Required");
        let mut image_data = vec![0u8; 2048];
        for y in 0..32 {
            for x in 0..64 {
                if state.gfx[y * 64 + x] == 1 {
                    image_data[(31 - y) * 64 + x] = u8::max_value();
                } else {
                    image_data[(31 - y) * 64 + x] = 0;
                }
            }
        }
        let target = display.draw();
        let screen = glium::texture::RawImage2d {
                      data: Cow::Borrowed(&image_data),
                      width: 64,
                      height: 32,
                      format: glium::texture::ClientFormat::U3U3U2};
        let opengl_texture = glium::Texture2d::new(display, screen).unwrap();
        
        opengl_texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        
        target.finish().unwrap();
    }
    update_timers(state);
    
    //println!("{:?}", state);
    //let mut line = String::new();
    // let _ = io::stdin().read_line(&mut line);
}

fn get_opcode(pc: usize, memory: &Vec<u8>) -> u16 {
    // Shift upper bits over 8 bits, then OR with lower 8 bits.
    u16::from(memory[pc]) << 8 | u16::from(memory[pc + 1])
}

fn execute_opcode(opcode: u16, state: &mut Chip8State, memory: &mut Vec<u8>) -> bool {
    let decode = opcode & 0xF000;
    let mut draw_flag = false;
    match decode {
        0x0000 => {
            if opcode == 0x00E0 {
                //println!("0x00E0 opcode (cls)");
                for x in 0..2048 {
                    state.gfx[x] = 0;
                }
                draw_flag = true;
            } else if opcode == 0x00EE {
                //println!("0x00EE opcode (return)");
                state.stack_pointer -= 1;
                state.pc = state.stack[state.stack_pointer];
            } else {
                println!("Unknown opcode: {:#X}", decode);
            }
            state.pc += 2;
        }
        0x1000 => {
            //println!("0x1 opcode (jmp)");
            state.pc = (opcode & 0x0FFF) as usize;
            }
        0x2000 => {
            //println!("0x2 opcode (call subroutine)");
            state.stack[state.stack_pointer] = state.pc;
            state.stack_pointer += 1;
            state.pc = (opcode & 0x0FFF) as usize;
            }
        0x3000 => {
            //println!("0x3 opcode (skip if equal)");
            if state.v[((opcode & 0x0F00) >> 8) as usize] == (opcode & 0x00FF) {
                state.pc += 2;
                }
            state.pc += 2;
            }
        0x4000 => {
            //println!("0x4 opcode (skip if not equal)");
            if state.v[((opcode & 0x0F00) >> 8) as usize] != (opcode & 0x00FF) {
                state.pc += 2;
                }
            state.pc += 2;
            }
        0x5000 => {
            //println!("0x5 opcode (skip if x = y)");
            if state.v[((opcode & 0x0F00) >> 8) as usize] == state.v[((opcode & 0x00F0) >> 4) as usize] {
                state.pc += 2;
                }
            state.pc += 2;
            }
        0x6000 => {
            //println!("0x6 opcode (set register)");
            state.v[((opcode & 0x0F00) >> 8) as usize] = opcode & 0x00FF;
            state.pc += 2;
            }
        0x7000 => {
            //println!("0x7 opcode (add to register)");
            state.v[((opcode & 0x0F00) >> 8) as usize] += opcode & 0x00FF;
            state.pc += 2;
            }
        0x8000 => {
            let operation = opcode & 0x000F;
            match operation {
                0x0000 => {
                    //println!("Set VX to VY");
                    state.v[((opcode & 0x0F00) >> 8) as usize] = state.v[((opcode & 0x00F0) >> 4) as usize];
                }
                0x0002 => {
                    //println!("Set VX to VX & VY");
                    state.v[((opcode & 0x0F00) >> 8) as usize] = state.v[((opcode & 0x0F00) >> 8) as usize] & state.v[((opcode & 0x00F0) >> 4) as usize];
                }
                0x0003 => {
                    //println!("Set VX to VX xor VY");
                    state.v[((opcode & 0x0F00) >> 8) as usize] = state.v[((opcode & 0x0F00) >> 8) as usize] ^ state.v[((opcode & 0x00F0) >> 4) as usize];
                }
                0x0004 => {
                    //println!("Add VY to VX, set overflow");
                    if state.v[((opcode & 0x00F0) >> 4) as usize] > (0xFF - state.v[((opcode & 0x0F00) >> 8) as usize]) {
                        state.v[0xF] = 1;
                    } else {
                        state.v[0xF] = 0;
                    }
                    state.v[((opcode & 0x0F00) >> 8) as usize] += state.v[((opcode & 0x00F0) >> 4) as usize];
                }
                0x0005 => {
                    //println!("Subtract VY from VX, set overflow");
                    if state.v[((opcode & 0x00F0) >> 4) as usize] > state.v[((opcode & 0x0F00) >> 8) as usize] {
                        state.v[0xF] = 0;
                        state.v[((opcode & 0x0F00) >> 8) as usize] = 0xFF - state.v[((opcode & 0x00F0) >> 4) as usize] + state.v[((opcode & 0x0F00) >> 8) as usize] + 1;
                    } else {
                        state.v[0xF] = 1;
                        state.v[((opcode & 0x0F00) >> 8) as usize] -= state.v[((opcode & 0x00F0) >> 4) as usize];
                    }
                }
                _ => {
                    println!("Unknown opcode: {:#X}", opcode);
                    }
                }
                state.pc +=2;
            }
        0x9000 => {
            //println!("Skip if VX = VY");
            if state.v[((opcode & 0x0F00) >> 8) as usize] == state.v[((opcode & 0x00F0) >> 4) as usize] {
                state.pc += 2;
            }
            state.pc += 2;
            }
        0xA000 => {
            //println!("0xA opcode (Set index)");
            state.index = opcode & 0x0FFF;
            state.pc += 2;
            }
        0xC000 => {
            state.v[((opcode & 0x0F00) >> 8) as usize] = (opcode & 0x00FF) & (rand::thread_rng().gen::<u16>() & 0xFF);
            state.pc += 2;
            }
        0xD000 => {
            //println!("Draw sprite");
            let x = state.v[((opcode & 0x0F00) >> 8) as usize];
            let y = state.v[((opcode & 0x00F0) >> 4) as usize];
            let height = opcode & 0x000F;
            
            // Reset carry flag
            state.v[0xF] = 0;
            
            for yline in 0..height {
                let pixel = memory[(state.index + yline) as usize];
                for xline in 0..8 {
                    if (pixel & (0x80 >> xline)) != 0 {
                        if state.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                            state.v[0xF] = 1;
                        }
                        state.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                    }
                }
            }
            draw_flag = true;
            state.pc += 2;
        }
        0xE000 => {
            let operation = opcode & 0x00FF;
            match operation {
                0x009E => {
                    //println!("Advance if key pressed");
                    if state.key_press[state.v[((opcode & 0x0F00) >> 8) as usize] as usize] == 1 {
                        state.pc += 2;
                    }
                }
                0x00A1 => {
                    //println!("Advance if key not pressed");
                    if state.key_press[state.v[((opcode & 0x0F00) >> 8) as usize] as usize] == 0 {
                        state.pc += 2;
                    }
                }
                _ => {
                    println!("Unknown opcode: {:#X}", opcode);
                    }
            }
            state.pc += 2;
        }
        0xF000 => {
            let operation = opcode & 0x00FF;
            let mut advance = true;
            match operation {
                0x0007 => {
                    //println!("Read delay timer");
                    state.v[((opcode & 0x0F00) >> 8) as usize] = state.delay_timer;
                }
                0x000A => {
                    //println!("Wait for keypress");
                    let mut pressed = false;
                    for x in 0..16 {
                        if state.key_press[x] == 1 {
                            pressed = true;
                            state.v[((opcode & 0x0F00) >> 8) as usize] = x as u16;
                            //println!("pressed");
                            break;
                        }
                    }
                    if !pressed {
                        advance = false;
                    }
                    }
                0x0015 => {
                    //println!("Set delay timer");
                    state.delay_timer = state.v[((opcode & 0x0F00) >> 8) as usize];
                    }
                0x001E => {
                    //println!("Add VX to I");
                    state.index += state.v[((opcode & 0x0F00) >> 8) as usize];
                    }
                0x0029 => {
                    //println!("Put sprite at index");
                    state.index = state.v[((opcode & 0x0F00) >> 8) as usize] * 5;
                    }
                0x0033 => {
                    //println!("Decimal representation");
                    memory[state.index as usize] = (state.v[((opcode & 0x0F00) >> 8) as usize] / 100) as u8;
                    memory[(state.index + 1) as usize] = ((state.v[((opcode & 0x0F00) >> 8) as usize] / 10) % 10) as u8;
                    memory[(state.index + 2) as usize] = ((state.v[((opcode & 0x0F00) >> 8) as usize] % 100) % 10) as u8;
                    }
                0x0055 => {
                    //println!("FX55 opcode");
                    let max = ((opcode & 0x0F00) >> 8) + 1;
                    for x in 0..max {
                        memory[(state.index + x) as usize] = state.v[x as usize] as u8;
                        }
                    }
                0x0065 => {
                    // Add one, because the for loop should be inclusive.
                    let max = ((opcode & 0x0F00) >> 8) + 1;
                    //println!("Fills {} registers from I pointer", max);
                    for x in 0..max {
                        state.v[x as usize] = memory[(state.index + x) as usize] as u16;
                        }
                    }
                _ => {
                    println!("Unknown opcode: {:#X}", opcode);
                    }
                }
            if advance {
                state.pc += 2;
            }
            }
        _ => {
            println!("Unknown opcode: {:#X}", opcode);
            state.pc += 2;
            }
    }
    draw_flag
}

fn update_timers(state: &mut Chip8State) {
    if state.delay_timer > 0 {
        state.delay_timer -= 1;
    }
    if state.sound_timer > 0 {
        state.sound_timer -= 1;
    }
}