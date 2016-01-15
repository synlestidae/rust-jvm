use bytecode::instruction::*;
use classfile::access_flags::*;
use classfile::javatype::*;
use classfile::MethodDescriptor;

#[derive(Debug, Clone)]
pub struct Method {
    pub code: Vec<u8>,
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: MethodDescriptor,
}
