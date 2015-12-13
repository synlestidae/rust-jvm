use classfile::attribute::Attribute;
use classfile::access_flags::AccessFlags;

pub struct Info {
    pub access_flags : AccessFlags, 
    pub name : String,
    pub descriptor : String,
    pub attributes : Vec<Attribute>
}