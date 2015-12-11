use classfile::attribute::Attribute;
use classfile::attribute::Attribute;

pub struct MethodInfo {
    access_flags : Flag, 
    name : String,
    descriptor : String,
    attributes : Vec<Attribute>
}