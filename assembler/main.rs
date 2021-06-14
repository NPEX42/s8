use s8_lib::asm::*;

#[cfg(not(feature = "libcore"))]
fn main() {
    let ins = chunk::Instruction::new(1,15,0,0,1,0);
    println!("{:?}", ins);

    let mut raw = unsafe {ins.as_mut_slice()};

    raw[0] = 0;

    println!("{:?}", raw);

    let new = chunk::Instruction::from_slice(&raw);

    assert_eq!(new, ins);
}