use classfile::info::*;
use classfile::attribute::*;
use classfile::constant::*;
use classfile::access_flags::*;

pub struct RefinedClassFile {
	pub minor_version : u16,
	pub major_version : u16,

	pub constant_pool_table : Vec<Constant>,
	
	pub access_flags : AccessFlags,
	pub this_class : String,	
	pub super_class : String,

	pub interface_table : Vec<u8>,
	pub field_table : Vec<Info>,
	pub method_table : Vec<Info>,
	pub attribute_table : Vec<Attribute>
}