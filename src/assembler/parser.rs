use crate::std::fmt::Display;

use crate::mem::vec::Vec;
use crate::mem::vec;
use crate::mem::string::String;
use crate::asm::lexer::Token;

pub type Mnemonic = String;

pub enum ArgType {
    Immediate(u8),
    Address(u16),
    Label(u16)
}

pub struct InstructionNode {
    mnem : Mnemonic,
    args : Vec<ArgType>
}

pub trait Parser<'a> { 
    fn parse(&mut self, tokens : Vec<Token>) -> &'a Vec<InstructionNode>;
}

#[derive(Debug)]
pub struct Program {
    program : Vec<Vec<Token>>
}

impl Program {
    pub fn parse(tokens : Vec<Token>) -> Self {
        let mut prg = Program {
            program : Vec::new()
        };

        prg._parse(tokens);

        prg
    }

    fn _parse(&mut self, tokens : Vec<Token>) {
        let mut active_line : Vec<Token> = Vec::new();
        for token in tokens {
            if token == Token::LineSep {
                self.program.append(&mut vec!(active_line));
                active_line = Vec::new();
            } else {
                active_line.push(token)
            }
        }
        self.program.append(&mut vec!(active_line));
    }

    pub fn lines(&self) -> &Vec<Vec<Token>> {
        &self.program
    }
    
}

impl Display for Program {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter<'_>) -> crate::std::fmt::Result {
        for token in &self.program {
            write!(f, "\t{:?}\n", token);
        }
        write!(f, "EOF")
    }
}

pub struct S8_Parser<'a> {
    nodes : &'a Vec<InstructionNode>,
    tokens : Vec<Token>,
    current : usize,
}

impl<'a> Parser<'a> for S8_Parser<'a> {
    fn parse(&mut self, tokens : Vec<Token>) -> &'a Vec<InstructionNode> {
        while !self.is_at_end() {
            let token = self.consume();
            match token {
                Token::Mnemonic(lbl) => {self.mnem(lbl)}
                _ => {}
            }
        }
        return &self.nodes;
    }
}

impl<'a> S8_Parser<'a> {
    fn next_matches(&self, mtch : Token) -> bool {
        if self.is_at_end() { return false }

        return self.tokens[self.current + 1] == mtch
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() <= self.current
    }

    fn consume(&mut self) -> &Token {
        let t = &self.tokens[self.current];
        self.current += 1;
        t 
    }

    #[cfg(feature = "libcore")]
    fn error(msg : String) {}

    #[cfg(not(feature = "libcore"))]
    fn error(msg : String) {
        panic!("{}",msg)
    }

    fn mnem(&self, mnem : &String) {

    }


    fn nop(&self) {

    }
}





impl InstructionNode {
    pub fn new(mnem : Mnemonic, args : Vec<ArgType>) -> Self {
        Self {
            mnem, args
        }
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnem
    }

    pub fn args(&self) -> &Vec<ArgType> {
        &self.args
    }
}