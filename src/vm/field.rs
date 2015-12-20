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
		match &self.field_type {
			&JavaType::Reference(_) => 8,
			&JavaType::Primitive(Primitive::Boolean) => 4,
			&JavaType::Primitive(Primitive::ReturnAddress) => 8,
			&JavaType::Primitive(Primitive::Numeric(ref num)) => match num {
				&Numeric::Integral(Integral::Byte) => 1,
				&Numeric::Integral(Integral::Short) => 2,
				&Numeric::Integral(Integral::Int) => 4,
				&Numeric::Integral(Integral::Long) => 8,
				&Numeric::Integral(Integral::Char) => 2,
				&Numeric::FloatingPoint(FloatingPoint::Float) => 4,
				&Numeric::FloatingPoint(FloatingPoint::Double) => 8
			}
		}
	}
}