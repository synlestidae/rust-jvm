use classfile::javatype::JavaType;
use std::ascii::*;
use vm::memory::heap_size::HeapSize;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MethodDescriptor {
    pub parameter_descriptors: Vec<FieldType>,
    pub return_descriptor: Option<FieldType>,
}

impl MethodDescriptor {
    pub fn parse_from(input_line: &str) -> Result<MethodDescriptor, String> {
        let line: Vec<char> = input_line.chars().collect();

        if line.len() <= 0 {
            return Err("Cannot parse empty string".to_string());
        }

        let mut i = 0;
        if line[i] != '(' {
            return Err("Expected '(' on method descriptor at index 0".to_string());
        }

        let mut parameters = Vec::new();
        while line[i] != ')' && i < line.len() {
            parameters.push(try!(JavaType::parse_field_type_partial(&line, &mut i)));
        }

        panic!("Not ready for this")
    }
}

pub type FieldDescriptor = FieldType;

impl FieldType {
    pub fn parse_from(s : &str) -> Result<FieldDescriptor, String> {
        let java_type = try!(JavaType::parse_field_type(s));

        Ok(match java_type {
            JavaType::Ref(class_name) => FieldType::Object(class_name),
            JavaType::ArrayRef(boxed_java_type, n) => FieldType::Array(Box::new(FieldType::Base(*boxed_java_type)), n),
            anything_else => FieldType::Base(anything_else)
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FieldType {
    Base(JavaType),
    Object(String),
    Array(Box<FieldType>, u8),
}

impl HeapSize for FieldType {
    fn size_of(&self) -> usize {
        match self {
            &FieldType::Base(ref java_type) => java_type.size(),
            &FieldType::Object(ref java_type) => 8,
            &FieldType::Array(ref java_type, _) => 8
        }
    }
}
