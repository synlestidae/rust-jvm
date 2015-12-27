#[derive(Debug, PartialEq, Eq)]
pub struct RawClassFile {
	pub minor_version : u16,
	pub major_version : u16,

	pub constant_pool_table : Vec<RawCpInfo>,
	
	pub access_flags : u16,
	pub this_class_index : u16,
	pub super_class_index : u16,

	pub interface_table : Vec<u8>,
	pub field_table : Vec<RawInfo>,
	pub method_table : Vec<RawInfo>,
	pub attribute_table : Vec<RawAttributeInfo>
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawCpInfo {
	pub tag : u8,
	pub additional_bytes : Vec<u8>
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawInfo {
	pub access_flags : u16,             
    pub name_index : u16, 
    pub descriptor_index : u16,
    pub attributes : Vec<RawAttributeInfo>
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawAttributeInfo {
	pub attribute_name_index : u16,
	pub info : Vec<u8>
}