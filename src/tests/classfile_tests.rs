use classfile_preprocessor::*;
use classfile::raw::*;
use classfile::classfile::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;


#[test]
fn smoke_test() {
	let mut cf = File::open("./src/tests/data/homemade/OneIntField.class").unwrap();
	let mut itWorked = false;

	if let Ok(raw_file) = read_class_file(&mut cf) {
		match refine_classfile(&raw_file) {
			Ok(cf) => println!("Holy shit it worked: {:?}", cf),
			Err(err) => println!("Yeah of course it didn't work: {}", err)
		}
	}
	else{
		assert!(false);
	}
}
