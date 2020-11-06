
pub fn xor_jump_9(data: &mut Vec<u8>, key : u8) {
    let mut ikey = key;
    for i in 0..data.len() {
        data[i] ^= key;
        ikey += 9;
    }
}
