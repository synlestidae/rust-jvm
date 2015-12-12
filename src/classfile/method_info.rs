use classfile::attribute::Attribute;
use classfile::access_flags::AccessFlags;

pub struct MethodInfo {
    access_flags : AccessFlags, 
    name : String,
    descriptor : String,
    attributes : Vec<Attribute>
}