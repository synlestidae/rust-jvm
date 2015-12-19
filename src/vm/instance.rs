use vm::Frame;
use vm::LoadedClassCollection;

pub struct Instance {
	frames : Vec<Frame>,
	classes : LoadedClassCollection
}

impl Instance {
	pub fn new() -> Instance {
		Instance {
			frames : Vec<Frame> 
		}
	} 
}