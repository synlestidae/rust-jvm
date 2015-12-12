use classfile::attribute::Attribute;
use classfile::access_flags::AccessFlags;

pub struct FieldInfo {
	access_flags : AccessFlags,
	name : String, 
	descriptor : String,
	attributes : Vec<Attribute>
}