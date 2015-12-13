use classfile::field_info::*;
use classfile::method_info::*;
use classfile::attribute::*;
use classfile::constant::*;
use classfile::access_flags::*;

pub struct RefinedClassFile {
	pub minor_version : u16,
	pub major_version : u16,

	pub constant_pool_table : Vec<Constant>,
	
	pub access_flags : AccessFlags,
	pub this_class_index : u16,
	pub super_class_index : u16,

	pub interface_table : Vec<u8>,
	pub field_table : Vec<FieldInfo>,
	pub method_table : Vec<MethodInfo>,
	pub attribute_table : Vec<Attribute>
}