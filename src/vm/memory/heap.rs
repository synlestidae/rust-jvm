use vm::heap_rep::HeapRep;

pub trait Heap {
	fn allocate(size : usize) -> usize;
	fn free(index : usize) -> Option<usize>;
	fn get<'a>(index : usize) -> &'a mut [u8];
	fn get_children(index : usize, rep : &HeapRep) -> Vec<usize>;
	fn maximum_size() -> usize;
	fn current_size() -> usize;
}