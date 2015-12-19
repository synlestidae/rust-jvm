use vm::Method;
use std::cmp::Eq;

#[derive(Clone, Debug)]
pub struct LoadedClass {
	pub public_methods : Vec<Method>,
	pub private_methods : Vec<Method>,
	pub name : String
}

impl PartialEq for LoadedClass {
	fn eq (self : &Self, other : &Self) -> bool {
		self.name == other.name
	} 
}

impl Eq for LoadedClass{}