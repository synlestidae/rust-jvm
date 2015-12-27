use classfile_preprocessor::*;
use classfile::raw::*;
use classfile::classfile::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use vm::memory::heap::*;
use vm::mempry::gc_heap::*;

#[test]
fn test_class_is_public_1() {
    let cf_path = Path::new("./src/tests/data/homemade/OneIntField.class");

    let heap = GcHeap::new(1024);

    if let Ok(cf) = load_classfile_from_file(cf_path) {

    } else {
        assert!(false);
    }
}
