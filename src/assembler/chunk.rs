use crate::std;
use crate::mem;

use mem::vec;
use mem::vec::Vec;

use mem::string::String;

#[derive(Debug)]
#[repr(C)]
pub struct Chunk {
    instructions    : Vec<Instruction>
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Instruction {
    id_opcode   : u8, //MSN ID, LSN Opcode
    arg_0       : u8,
    arg_1       : u8,
    arg_2       : u8,
    arg_3       : u8,
}

impl Instruction {
    pub fn new(set_id : u8, opcode : u8, arg_0 : u8, arg_1 : u8, arg_2 : u8, arg_3 : u8) -> Self {
        Self {
            id_opcode : ((set_id & 0x0F) << 4) | (opcode & 0x0F),
            arg_0,
            arg_1,
            arg_2,
            arg_3
        }
    }

    pub fn from_slice(slice : &[u8]) -> Self {
        assert_eq!(slice.len(), std::mem::size_of::<Instruction>());
        Self {
            id_opcode : slice[0],
            arg_0 : slice[1],
            arg_1 : slice[2],
            arg_2 : slice[3],
            arg_3 : slice[4]
        }
    }

    pub unsafe fn as_mut_slice<'a>(&mut self) -> &mut [u8]  {
        let mut view = self as *mut _ as *mut u8;
        let slice = std::slice::from_raw_parts_mut(view, std::mem::size_of::<Instruction>());
        slice
    }

} 