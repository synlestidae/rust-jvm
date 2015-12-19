mod rawprocessor;
mod refiner;
mod refine_constant;
mod refine_attribute;

use self::rawprocessor::{read_class_file};
use self::refiner::{refine_classfile};

use std::path::Path;
use self::refiner::{ClassFileProcessingError};

use classfile::classfile::RefinedClassFile;

pub fn load_classfile_from_bytes(bytes : &[u8]) 
	-> Result<RefinedClassFile, ClassFileProcessingError> {
	panic!("Not done yet");
}

pub fn load_classfile_from_path(path : &Path) 
	-> Result<RefinedClassFile, ClassFileProcessingError> {
	match File::open(path) {
		Ok(_) => panic!("Oh shoot!"),
		Err(_) => format!("Cannot load class file from this path as it doesn't exist: {}", path)
	}
}