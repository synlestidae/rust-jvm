use classfile::javatype::JavaType;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MethodDescriptor {
    pub parameter_descriptors: Vec<FieldType>,
    pub return_descriptor: Option<FieldType>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FieldType {
    Base(JavaType),
    Object(String),
    Array(Box<FieldType>),
}
