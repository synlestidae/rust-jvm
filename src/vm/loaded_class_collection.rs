use vm::LoadedClass;

pub struct LoadedClasses {
	classes : Vec<LoadedClass>
}

impl LoadedClasses {
	pub fn find_class(self : &Self, class_name : &str) -> Option<LoadedClass> {
		for class in self.classes.iter() {
			if &class.name == class_name {
				return Some(class.clone())
			}
		}
		None
	}

	pub fn add_class(self : &mut Self, class : LoadedClass) {
		self.classes.push(class)
	}
}