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

	match load_classfile_from_bytes(&raw_file) {
		Ok(cf) => (),
		Err(err) => assert!(false)
	}
	else{
		assert!(false);
	}
}

#[test]
fn smoke_test_2() {
	let mut cf = File::open("./src/tests/data/homemade/OneVoidMethod.class").unwrap();
	let mut itWorked = false;

	if let Ok(raw_file) = read_class_file(&mut cf) {
		match refine_classfile(&raw_file) {
			Ok(cf) => (),
			Err(err) => assert!(false)
		}
	}
	else{
		assert!(false);
	}
}

#[test]
fn smoke_test_3() {
	let mut cf = File::open("./src/tests/data/homemade/Empty.class").unwrap();
	let mut itWorked = false;

	if let Ok(raw_file) = read_class_file(&mut cf) {
		match refine_classfile(&raw_file) {
			Ok(cf) => (),
			Err(err) => println!("Yeah of course it didn't work: {}", err)
		}
	}
	else{
		assert!(false);
	}
}

#[test]
fn test_class_is_public_1() {
	let mut cf = File::open("./src/tests/data/homemade/Empty.class").unwrap();
	let mut itWorked = false;

	if let Ok(cf) = refine_classfile(&read_class_file(&mut cf).unwrap()) {
		assert!(cf.access_flags.ACC_PUBLIC);
		assert!(!cf.access_flags.ACC_PROTECTED);
		assert!(!cf.access_flags.ACC_STATIC);
		assert!(!cf.access_flags.ACC_FINAL);
		assert!(cf.access_flags.ACC_SUPER);
		assert!(!cf.access_flags.ACC_BRIDGE);
		assert!(!cf.access_flags.ACC_VARARGS);
		assert!(!cf.access_flags.ACC_NATIVE);
		assert!(!cf.access_flags.ACC_ABSTRACT);
		assert!(!cf.access_flags.ACC_STRICT);
		assert!(!cf.access_flags.ACC_SYNTHETIC);
	}
	else{
		assert!(false);
	}	
}