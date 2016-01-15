use vm::Method;
use vm::Field;
use vm::ClassLoader;
use vm::memory::representation::Representation;
use vm::memory::heap_size::HeapSize;

use std::cmp::Eq;
use std::collections::HashMap;

use classfile::classfile::RefinedClassFile;
use classfile::javatype::*;
use classfile::access_flags::AccessFlags;
use classfile::info::*;
use classfile::MethodDescriptor;
use classfile::attribute::Attribute;

#[derive(Clone, Debug)]
pub struct Class {
    name: String,
    super_class_name: String,
    interface_class_names: Vec<String>,
    fields: HashMap<String, (Field, usize)>,
    methods: HashMap<String, Method>,
    access_flags: AccessFlags,
    heap_size: usize,
}

pub enum ClassCreationError {
    InvalidFormat(String),
}

impl Class {
    pub fn from(class_file: RefinedClassFile) -> Result<Class, ClassCreationError> {
        let fields = fields_from_classfile(&class_file);
        let methods = methods_from_classfile(&class_file);
        let name = class_file.this_class;
        let super_name = class_file.super_class;

        let mut fields_map = HashMap::new();
        let mut methods_map = HashMap::new();

        let mut sum = 0;
        for f in fields.into_iter() {
            sum = sum + f.size_of();
            let name = f.name.clone();
            fields_map.insert(name, (f, sum));
        }

        for m in methods.into_iter() {
            methods_map.insert(m.name.clone(), m);
        }

        Ok(Class {
            name: name,
            super_class_name: super_name,
            interface_class_names: Vec::new(),
            fields: fields_map,
            methods: methods_map,
            access_flags: class_file.access_flags,
            heap_size: sum,
        })
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

fn fields_from_classfile(class_file: &RefinedClassFile) -> Vec<Field> {
    class_file.field_table.iter().map(|ref f| make_field(&f)).collect()
}

fn methods_from_classfile(class_file: &RefinedClassFile) -> Vec<Method> {
    class_file.method_table.iter().map(|ref f| make_method(&f)).collect()
}

fn make_field(field_info: &Info) -> Field {
    let field_descriptor = JavaType::parse_field_type(&field_info.descriptor).unwrap();

    Field {
        flags : field_info.access_flags,
        name : field_info.name.clone(),
        field_type : field_descriptor.clone()
    }
}

fn make_method(method_info: &Info) -> Method {
    let descriptor = MethodDescriptor::parse_from(
            &method_info.descriptor)
        .unwrap();

    //scan for a code attribute
    for attribute in &method_info.attributes {
        match attribute {
            &Attribute::Code {
                max_stack: _,
                max_locals: _,
                code: ref attr_code,
                exception_table: _,
                attributes: _,

            } => {
                return Method {
                    access_flags : method_info.access_flags,
                    name : method_info.name.clone(),
                    descriptor : descriptor,
                    code : attr_code.clone()
                }
            }
            //continue if nothing found
            _ => {}
        }
    }

    panic!("Method's Info object doesn't have a code attribute");
}
