use classfile_preprocessor::*;
use classfile::raw::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;


#[test]
fn smoke_test() {
	let mut cf = File::open("./src/tests/data/homemade/OneIntField.class").unwrap();
	let mut itWorked = false;

	if let Err(_) = read_class_file(&mut cf) {
		assert!(false);
	}
}

#[test]
fn test_major_minor_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/Empty.class")
		.unwrap()).unwrap();
	assert_eq!(52, classfile.major_version);
	assert_eq!(0, classfile.minor_version);
}

#[test ]
fn test_constant_pool_correct_size_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/Empty.class")
		.unwrap()).unwrap();
	assert_eq!(12, classfile.constant_pool_table.len());
}

#[test]
fn test_constant_pool_correct_tags_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/Empty.class")
		.unwrap()).unwrap();
	assert_eq!(10, classfile.constant_pool_table[0].tag);
	assert_eq!(7, classfile.constant_pool_table[1].tag);
	assert_eq!(7, classfile.constant_pool_table[2].tag);
	assert_eq!(1, classfile.constant_pool_table[4].tag);
	assert_eq!(1, classfile.constant_pool_table[3].tag);
	assert_eq!(1, classfile.constant_pool_table[5].tag);
	assert_eq!(1, classfile.constant_pool_table[6].tag);
	assert_eq!(1, classfile.constant_pool_table[7].tag);
	assert_eq!(1, classfile.constant_pool_table[8].tag);
	assert_eq!(12, classfile.constant_pool_table[9].tag);
	assert_eq!(1, classfile.constant_pool_table[10].tag);
	assert_eq!(1, classfile.constant_pool_table[11].tag);
}

#[test]
fn test_superclass_index_has_class_tag_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/Empty.class")
		.unwrap()).unwrap();

	let super_entry = &classfile.constant_pool_table[classfile.super_class_index
			as usize - 1];

	assert_eq!(7, super_entry.tag);
}

#[test]
fn test_thisclass_index_has_class_tag_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/Empty.class")
		.unwrap()).unwrap();

	let super_entry = &classfile.constant_pool_table[classfile.this_class_index
			as usize - 1];

	assert_eq!(7, super_entry.tag);
}

#[test]
fn test_one_field_1() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/OneIntField.class")
			.unwrap()).unwrap();	
	assert_eq!(1, classfile.field_table.len());
}

#[test]
fn test_field_has_name_and_correct_type() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/OneIntField.class")
			.unwrap()).unwrap();
	let field_info = &classfile.field_table[0];
	let field_const = &classfile.constant_pool_table[field_info.name_index 
		as usize];

	assert_eq!(1, field_const.tag);
}

#[test]
fn test_constructor_method_in_table() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/OneVoidMethod.class")
			.unwrap()).unwrap();
	
	let constructor_method_info = &classfile.method_table[0];

	let mut name_index = constructor_method_info.name_index;
	let constant = classfile.constant_pool_table[name_index 
		as usize - 1].clone();
	assert_eq!(1, constant.tag);
	assert_eq!("<init>".to_string(), 
		String::from_utf8(constant.additional_bytes).unwrap());
}

#[test]
fn test_void_method_in_table() {
	let classfile = read_class_file(&mut File::open("./src/tests/data/homemade/OneVoidMethod.class")
			.unwrap()).unwrap();
	
	let void_method_info = &classfile.method_table[1];
	let mut name_index = void_method_info.name_index;

	let constant = classfile.constant_pool_table[name_index 
		as usize - 1].clone();
	assert_eq!(1, constant.tag);
	assert_eq!("voidMethod".to_string(), 
		String::from_utf8(constant.additional_bytes).unwrap());
}