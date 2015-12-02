pub struct ClassFile {
	minor_version : i16,
	major_version : i16,

	constant_pool_table : Vec<CpInfo>,
	
	access_flags : i16,
	this_class_index : i16,
	super_class_index : i16,

	interface_table : Vec<u8>,
	field_table : Vec<FieldInfo>,
	method_table : Box<MethodInfo>,
	attribute_table : Vec<AttributeInfo>
} 

pub struct CpInfo {
	pub tag : u8,
	pub additional_bytes : Vec<u8>
}

pub struct FieldInfo {
	access_flags : i16,             
    name_index : i16, 
    descriptor_index : i16,
    attributes : Vec<AttributeInfo>
}

pub struct AttributeInfo {
	attribute_name_index : i16,
	info : Vec<u8>
}

pub struct FieldInfo {

}