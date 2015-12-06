#[derive(Debug, PartialEq)]
pub struct ClassFile {
	pub minor_version : u16,
	pub major_version : u16,

	pub constant_pool_table : Vec<CpInfo>,
	
	pub access_flags : u16,
	pub this_class_index : u16,
	pub super_class_index : u16,

	pub interface_table : Vec<u8>,
	pub field_table : Vec<FieldInfo>,
	pub method_table : Vec<MethodInfo>,
	pub attribute_table : Vec<AttributeInfo>
} 

#[derive(Debug, PartialEq)]
pub struct CpInfo {
	pub tag : u8,
	pub additional_bytes : Vec<u8>
}

pub type FieldInfo = Info;
pub type MethodInfo = Info;

#[derive(Debug, PartialEq)]
pub struct Info {
	pub access_flags : u16,             
    pub name_index : u16, 
    pub descriptor_index : u16,
    pub attributes : Vec<AttributeInfo>
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
	pub attribute_name_index : u16,
	pub info : Vec<u8>
}