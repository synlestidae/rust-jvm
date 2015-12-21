use vm::memory::representation::*;
use vm::class::*;
use classfile::javatype::*;

pub trait Heap {
	fn allocate_object(self : &mut Self, class : &Class) -> Option<usize>;
	fn get<'a>(self : &mut Self, index : usize) -> Option<&'a mut [u8]>;
	fn maximum_size(self : &Self, ) -> usize;
	fn current_size(self : &Self, ) -> usize;
	fn garbage_collect() -> usize;
}