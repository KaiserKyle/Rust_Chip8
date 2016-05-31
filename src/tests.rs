use super::get_opcode;

#[test]
fn test_opcode_read() {
    let mut v: Vec<u8> = Vec::new();
    v.push(0x12);
    v.push(0x34);
    assert_eq!(0x1234, get_opcode(0, &v));
}