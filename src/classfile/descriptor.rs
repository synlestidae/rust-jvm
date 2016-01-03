use classfile::javatype::JavaType;
use std::ascii::*;

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
            parameters.push(try!(parse_field_type(&line, &mut i)));
        }

        panic!("Not ready for this")
    }
}

fn parse_field_type(line: &[char], i: &mut usize) -> Result<JavaType, String> {
    let t = match line[*i] {
        'B' => JavaType::Byte,
        'C' => JavaType::Char,
        'D' => JavaType::Double,
        'F' => JavaType::Float,
        'I' => JavaType::Int,
        'J' => JavaType::Long,
        'L' => panic!("Not ready for this"),
        'S' => JavaType::Short,
        'Z' => JavaType::Int,
        '[' => panic!("Not ready for this"),
        _ => return Err(format!("Unexpected character '{}' at index {}", i, line[*i])),
    };
    *i += 1;
    Ok(t)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FieldType {
    Base(JavaType),
    Object(String),
    Array(Box<FieldType>),
}
