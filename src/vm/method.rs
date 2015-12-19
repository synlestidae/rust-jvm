use bytecode::instruction::*;
use classfile::access_flags::*;

#[derive(Debug, Clone)]
pub struct Method {
	pub code : Vec<Instruction>,
	pub flags : AccessFlags,
	pub name : String 
}