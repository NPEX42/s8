use crate::asm::parser::Program;
use crate::asm::lexer::Token;
use crate::mem::vec::Vec;
use crate::mem::string::String;
use crate::mem::collections::BTreeMap;

use crate::asm::*;

pub trait CodeGenerator<'a> {
    fn generate(&mut self, prg : &'a Program) -> &Vec<u8>;
}

pub struct S8_Compiler<'a> {
    labels : BTreeMap<&'a String, u16>,
    bin : Vec<u8>,
    pad : usize
}

impl<'a> CodeGenerator<'a> for S8_Compiler<'a> {
    fn generate(&mut self, prg : &'a Program) -> &Vec<u8> {
       let mut line_num = 0;
       for line in prg.lines() {
           self.pad = 0;
           for token in line {
                match token {
                    Token::Mnemonic(mnem) => {self.emit_u8(S8_Compiler::lookup_mnem(mnem))}
                    Token::Register(ind) => {self.emit_u8(*ind as u8)}
                    Token::Immediate(val) => {self.emit_u8(*val as u8)}
                    Token::Address(addr) => {self.emit_u16(*addr as u16)}
                    Token::Label(lbl) => {self.emit_label(lbl)}
                    Token::LabelDef(lbl) => {self.define_label(lbl, line_num * INSTRUCTION_SIZE as u16)}
                    Token::LineSep => {}
                    Token::Whitespace => {}
                    Token::EndOfFile => {}
                }
           }

           for _ in self.pad .. INSTRUCTION_SIZE {
                self.emit_u8(0)
           }
       }
       assert!(self.bin.len() % INSTRUCTION_SIZE == 0);
       &self.bin
    }



    
}

impl<'a> S8_Compiler<'a> {

    pub fn new() -> Self {
        Self {
            labels : BTreeMap::new(),
            bin : Vec::new(),
            pad : 0
        }
    }

    fn define_label(&mut self, lbl : &'a String, index : u16) {
        #[cfg(not(feature = "libcore"))]
            println!("defining Label: {} 0x{:04x}", lbl, index);
        self.labels.insert(lbl, index);
    }

    fn emit_label(&mut self, lbl : &String) {
        #[cfg(not(feature = "libcore"))]
            println!("emitting Label: {}", lbl);
        self.emit_u16(*self.labels.get(lbl).unwrap_or(&0))
    }

    fn emit_u8(&mut self, x : u8) {
        #[cfg(not(feature = "libcore"))]
            println!("Emitting u8: {:02x}", x);
        self.bin.push(x);
        self.pad += 1;
    }

    fn emit_u16(&mut self, x : u16) {
        #[cfg(not(feature = "libcore"))]
            println!("Emitting u16: {:04x}", x);
        self.bin.push(((x & 0xFF00) >> 8) as u8);
        self.bin.push(((x & 0x00FF) >> 0) as u8);

        self.pad += 2;
    }


    fn lookup_mnem(mnem : &String) -> u8 {
        match mnem.as_str() {
 
            "NOP" => 0,
            "INT" => 1,
            "ADD" => 2,
            "SUB" => 3,
            "AND" => 4,
            "NOT" => 5,
            "JMZ" => 6,
            "JNZ" => 7,
            "LDA" => 8,
            "LDI" => 9,
            "BSL" => 10,
            "BSR" => 11,
            "POP" => 12,
            "PUSH" => 13,
            "STA" => 14,
            "HLT" => 15,

            _ => 0,
        }
    }
}