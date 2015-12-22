use bytecode::instruction::*;
use classfile::access_flags::*;
use classfile::javatype::*;

#[derive(Debug, Clone)]
pub struct Method {
	pub code : Vec<u8>,
	pub flags : AccessFlags,
	pub name : String,
	pub parameters : Vec<JavaType>,
	pub return_type : JavaType
}