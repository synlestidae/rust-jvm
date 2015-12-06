use classfile::*;
use class_file_reader::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;


#[test]
fn smoke_test() {
	println!("Working in: {}", env::current_dir().unwrap().display());
	let mut cf = File::open("./src/tests/data/homemade/Empty.class").unwrap();

	match read_class_file(&mut cf) {
		Ok(_) => println!("Holy shit it worked"),
		Err((errIndex, errMessage)) => println!("Yeah this is more likely: it fail son at {}: {}", errIndex, errMessage)
 	}
}