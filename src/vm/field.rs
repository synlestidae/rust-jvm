use classfile::access_flags::*;
use classfile::javatype::*;
use vm::memory::*;
use vm::memory::heap_size::HeapSize;

#[derive(Clone, Debug)]
pub struct Field {
	pub flags : AccessFlags,
	pub name : String,
	pub field_type : JavaType
}

impl HeapSize for Field {
	fn size_of(self : &Self) -> usize {
		self.field_type.size()
	}
}