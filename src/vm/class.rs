use vm::Method;
use vm::Field;
use vm::ClassLoader;
use vm::memory::representation::Representation;
use vm::memory::heap_size::HeapSize;
use std::cmp::Eq;
use std::collections::HashMap;
use classfile::classfile::RefinedClassFile;
use classfile::javatype::*;

#[derive(Clone, Debug)]
pub struct Class {
    name: String,
    super_class: Box<Class>,
    heap_size: usize,
    fields: HashMap<String, (Field, usize)>,
    methods: HashMap<String, Method>,
}

impl Class {
    fn new(name: &str, fields: Vec<Field>, methods: Vec<Method>, super_class: Class) -> Class {
        let mut fields_map = HashMap::new();
        let mut methods_map = HashMap::new();

        let mut sum = 0;
        for f in fields.into_iter() {
            sum = sum + f.size_of();
            fields_map.insert(f.name(), (f, sum));
        }

        for m in methods.into_iter() {
            methods_map.insert(m.name.clone(), m);
        }

        Class {
            name: name.to_string(),
            super_class: Box::new(super_class),
            heap_size: sum,
            fields: fields_map,
            methods: methods_map,
        }
    }

    pub fn name(self: &Self) -> String {
        self.name.clone()
    }
}

impl Representation for Class {
    fn total_size(self: &Self) -> usize {
        self.heap_size
    }

    fn field_offset(self: &Self, field_name: &str) -> Option<usize> {
        match self.fields.get(field_name) {
            Some(&(_, offset)) => Some(offset),
            None => None,
        }
    }
}

impl PartialEq for Class {
    fn eq(self: &Self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Class {}

pub type ClassCreationError = String;
