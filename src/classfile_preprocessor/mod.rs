mod rawprocessor;
mod refiner;
mod refine_constant;
mod refine_attribute;

use self::rawprocessor::{read_class_file};
use self::refiner::{refine_classfile};
use std::fs::File;

use std::path::Path;
use self::refiner::{ClassFileProcessingError};

use std::io;
use std::io::{Read};

use classfile::raw::RawClassFile;
use classfile::classfile::RefinedClassFile;

pub fn load_classfile_from_bytes(bytes : &[u8]) 
	-> Result<RefinedClassFile, ClassFileProcessingError> {
	panic!("Not implemented");
}

pub fn load_classfile_from_file(path : &Path) 
	-> Result<RefinedClassFile, ClassFileProcessingError> {
	
	if let Ok(ref mut file) = File::open(path) {
		let raw_classfile = match read_class_file(file) {
			Ok (rcf) => rcf,
			Err((_, error_message)) => return Err(error_message)
		};

		refine_classfile(&raw_classfile)
	}
	else {
		Err(format!("Could not open {}", path.display()))
	}
}