pub mod chunk;
pub mod lexer;
pub mod parser;
pub mod codegen;

pub const INSTRUCTION_SIZE : usize = crate::std::mem::size_of::<chunk::Instruction>();