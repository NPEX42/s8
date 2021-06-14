use std::io::Write;

use s8_lib::{asm::*, assembler::codegen::CodeGenerator};

#[cfg(not(feature = "libcore"))]
fn main() {




    let tokens = lexer::scan(&*String::from_utf8_lossy(std::fs::read("hello.asm").expect("Unable To Load file...").as_slice()));
    println!("{:?}", tokens);

    let prg = parser::Program::parse(tokens);

    println!("{}", prg);


    let mut compiler = codegen::S8_Compiler::new();
    let bin = compiler.generate(&prg);
    println!("{:?}", bin);

    let mut file = std::fs::File::create("hello.bin").expect("Unable To open file...");
    file.write_all(bin).expect("Couldn't Write To File...");
}

#[cfg(feature = "libcore")]
fn main() {
}