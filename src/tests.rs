use super::get_opcode;
use super::execute_opcode;
use super::Chip8State;
use super::init_state;

#[test]
fn test_opcode_read() {
    let mut v: Vec<u8> = Vec::new();
    v.push(0x12);
    v.push(0x34);
    assert_eq!(0x1234, get_opcode(0, &v));
}

#[test]
fn test_opcode_00e0() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    // Fill graphics with non-zero dat
    state.gfx = vec![3u8; 2048];
    execute_opcode(0x00E0, &mut state, &mut memory);
    
    // gfx should be all zeroes
    for x in 0..2048 {
        assert_eq!(0, state.gfx[x]);
    }
}

#[test]
fn test_opcode_00ee() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    // set a return address on the stack
    state.stack[0] = 0xabcd;
    state.stack_pointer = 1;
    
    execute_opcode(0x00EE, &mut state, &mut memory);
    
    // program counter should be one opcode ahead of the return address 
    assert_eq!(0xabcd + 2, state.pc);
}

#[test]
fn test_opcode_1nnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    execute_opcode(0x1858, &mut state, &mut memory);
    
    // pc should be at jump address
    assert_eq!(0x0858, state.pc);
}

#[test]
fn test_opcode_2nnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.pc = 0x8f3e;
    execute_opcode(0x2858, &mut state, &mut memory);
    
    // pc should be at jump address
    assert_eq!(0x0858, state.pc);
    // stack should have return address
    assert_eq!(1, state.stack_pointer);
    assert_eq!(0x8f3e, state.stack[0]);
}

#[test]
fn test_opcode_3xnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x34;
    state.pc = 0x3456;
    execute_opcode(0x3334, &mut state, &mut memory);
    
    // we should skip a step
    assert_eq!(0x345a, state.pc);
    
    execute_opcode(0x33ab, &mut state, &mut memory);
    
    // we should not skip a step
    assert_eq!(0x345c, state.pc);
}

#[test]
fn test_opcode_4xnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x34;
    state.pc = 0x3456;
    execute_opcode(0x4334, &mut state, &mut memory);
    
    // we should not skip a step
    assert_eq!(0x3458, state.pc);
    
    execute_opcode(0x43ab, &mut state, &mut memory);
    
    // we should not skip a step
    assert_eq!(0x345c, state.pc);
}

#[test]
fn test_opcode_5xyn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x45;
    state.v[5] = 0x28;
    state.v[8] = 0x45;
    state.pc = 0x3456;
    execute_opcode(0x5350, &mut state, &mut memory);
    
    // we should not skip a step
    assert_eq!(0x3458, state.pc);
    
    execute_opcode(0x5380, &mut state, &mut memory);
    
    // we should not skip a step
    assert_eq!(0x345c, state.pc);
}

#[test]
fn test_opcode_6xnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x45;
    execute_opcode(0x63ff, &mut state, &mut memory);
    
    // v3 should have updated value
    assert_eq!(0xff, state.v[3]);
}

#[test]
fn test_opcode_7xnn() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x45;
    state.v[0xf] = 0x00;
    execute_opcode(0x7303, &mut state, &mut memory);
    
    // v3 should have updated value
    assert_eq!(0x48, state.v[3]);
    assert_eq!(0x00, state.v[0xf]);
    
    execute_opcode(0x73ff, &mut state, &mut memory);
    
    // overflow gracefully, but do not set v[f]
    assert_eq!(0x47, state.v[3]);
    assert_eq!(0x00, state.v[0xf]);
    
    execute_opcode(0x73c3, &mut state, &mut memory);
    assert_eq!(0x0a, state.v[3]);
    assert_eq!(0x00, state.v[0xf]);
}

#[test]
fn test_opcode_8xy0() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[3] = 0x45;
    state.v[6] = 0xff;
    execute_opcode(0x8630, &mut state, &mut memory);
    
    // v6 should have updated value
    assert_eq!(0x45, state.v[6]);
    assert_eq!(0x45, state.v[3]);
}

#[test]
fn test_opcode_8xy1() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[7] = 0x76;
    state.v[2] = 0x5a;
    execute_opcode(0x8271, &mut state, &mut memory);
    
    // v6 should have updated value
    assert_eq!((0x5a | 0x76), state.v[2]);
    assert_eq!(0x76, state.v[7]);
}

#[test]
fn test_opcode_8xy2() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[7] = 0x76;
    state.v[2] = 0x5a;
    execute_opcode(0x8272, &mut state, &mut memory);
    
    // v6 should have updated value
    assert_eq!((0x5a & 0x76), state.v[2]);
    assert_eq!(0x76, state.v[7]);
}

#[test]
fn test_opcode_8xy3() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[7] = 0x76;
    state.v[2] = 0x5a;
    execute_opcode(0x8273, &mut state, &mut memory);
    
    // v6 should have updated value
    assert_eq!((0x5a ^ 0x76), state.v[2]);
    assert_eq!(0x76, state.v[7]);
}

#[test]
fn test_opcode_8xy4() {
    let mut state: Chip8State = Default::default(); 
    let mut memory = vec![0u8; 4096];
    init_state(&mut state);
    
    state.v[7] = 0x76;
    state.v[2] = 0xfe;
    execute_opcode(0x8274, &mut state, &mut memory);
    
    // v6 should have updated value
    assert_eq!(0x74, state.v[2]);
    assert_eq!(0x76, state.v[7]);
    assert_eq!(0x01, state.v[0xf]);
    
    state.v[0xa] = 0x02;
    execute_opcode(0x82a4, &mut state, &mut memory);
    
    assert_eq!(0x76, state.v[2]);
    assert_eq!(0x02, state.v[0xa]);
    assert_eq!(0x00, state.v[0xf]);
}