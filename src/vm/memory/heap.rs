use vm::memory::representation::*;
use classfile::javatype::*;

pub trait Heap {
	fn allocate(self : &mut Self, java_type : JavaType) -> Option<usize>;
	fn get<'a>(self : &mut Self, index : usize) -> Option<&'a mut [u8]>;
	fn maximum_size(self : &Self, ) -> usize;
	fn current_size(self : &Self, ) -> usize;
}