use regex_lexer::LexerBuilder;

use crate::std;
use crate::mem;

use mem::vec;
use mem::vec::Vec;

use mem::string::String;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Mnemonic(String),
    Register(usize),
    Immediate(usize),
    Address(usize),
    Label(String),
    LabelDef(String),
    LineSep,
    Whitespace,
    EndOfFile,
}

pub fn scan(src : &str) -> Vec<Token> {
    let lxr = LexerBuilder::new()
    .token(r"[A-z]+", |s| {Some(Token::Mnemonic(String::from(s)))})
    .token(r"r[0-9A-Fa-f]{2}", |s| {Some(Token::Register(usize::from_str_radix(&s[1..], 10).expect("Failed To Parse Reg Value")))})
    .token(r"\$[0-9A-Fa-f]{4}", |s| {Some(Token::Address(usize::from_str_radix(&s[1..], 16).expect("Failed To Parse Hex Value")))})
    .token(r"#[0-9]{1,3}", |s| {Some(Token::Immediate(usize::from_str_radix(&s[1..], 10).expect("Failed To Parse Dec Value")))})
    .token(r"[A-z][A-z0-9:]+:", |s| {Some(Token::LabelDef(String::from(&s[..s.len() -1 ])))})
    .token(r"@[A-z][A-z0-9:]+", |s| {Some(Token::Label(String::from(&s[1..])))})
    .token(r"[\t ]+", |_| {None})
    .token(r"[\r\n]+", |_| {Some(Token::LineSep)})
    .build().expect("Unable to Build Parser");

    let mut toks : Vec<Token> = lxr.tokens(src).collect();
    toks.push(Token::EndOfFile);
    toks
}