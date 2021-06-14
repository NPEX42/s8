use crate::std::fmt::Display;

use crate::mem::vec::Vec;
use crate::mem::vec;
use crate::asm::lexer::Token;

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