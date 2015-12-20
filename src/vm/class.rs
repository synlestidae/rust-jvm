use vm::Method;
use vm::Field;
use vm::ClassLoader;
use vm::memory::representation::Representation;
use vm::memory::heap_size::HeapSize;
use std::cmp::Eq;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Class {
	public_methods : Vec<Method>,
	private_methods : Vec<Method>,
	public_fields : Vec<Field>,
	private_fields : Vec<Field>,
	name : String,
	super_class : Box<Class>,

	heap_size : usize,
	fields : HashMap<String, usize>
}

impl Class {
	fn initialize(&mut self) {
		let mut sum = 0;

		for f in self.public_fields.iter() {
			sum += f.size_of();
		}

		for f in self.private_fields.iter() {
			sum += f.size_of();
		}

		self.heap_size = sum;
	}

	pub fn name(self : &Self) -> String {
		self.name.clone()
	}
}

impl Representation for Class {
	fn total_size(self : &Self) -> usize {
		self.heap_size
	}

	fn private_field_offset_unsafe(self : &Self, field_name : &str) -> usize {
		self.fields.get(field_name).unwrap().clone()
	}

	fn public_field_offset_unsafe(self : &Self, field_name : &str) -> usize {
		self.fields.get(field_name).unwrap().clone()
	}

	fn private_field_offset(self : &Self, field_name : &str) -> Option<usize> {
		match self.fields.get(field_name) {
			Some(&u) => Some(u),
			None => None
		}
	}

	fn public_field_offset(self : &Self, field_name : &str) -> Option<usize> {
		self.private_field_offset(field_name)
	}
}


impl PartialEq for Class {
	fn eq (self : &Self, other : &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Class{}