use classfile_preprocessor::*;
use classfile::raw::*;
use classfile::classfile::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

#[test]
fn test_class_is_public_1() {
	let mut cf_path = Path::new("./src/tests/data/homemade/OneIntField.class");

	if let Ok(cf) = load_classfile_from_file(cf_path) {
		println!("CF:\n{:?}", cf);
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
	else {
		assert!(false);
	}	
}