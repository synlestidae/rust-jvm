use std::io::Read;
use classfile::*;

pub type ReadError = (usize, String);

pub fn read_class_file(source : &mut Read) -> Result<ClassFile, ReadError> {
	let mut buf = Vec::new();

	source.read_to_end(&mut buf);

	if buf.len() < 4 {
		return Err((buf.len() - 1, "Expected magic number. Class file too short.".to_string()))
	}

	if buf.len() < 8 {
		return Err((buf.len() - 1, "Expected major/minor version. Class file too short.".to_string()))
	}

	let major_version = buf[7];
	let minor_version = buf[8];

	if buf.len() < 10 {
		return Err((buf.len() - 1, "Expected constant pool offset. Class file too short.".to_string()))
	}

	//parse the variable-length constant pool
	let constant_pool_count = (buf[8] as i16) << 8 + (buf[9] as i16);
	let mut index = 10;
	let mut constant_pool : Vec<CpInfo> = Vec::new();
	for i in 0..constant_pool_count {
		let cp_info_entry = try!(read_constant_pool_entry(&mut buf, &mut index));
		constant_pool.push(cp_info_entry);
	}

	let cpsize = (index + 1) - 10;

	/** 
	10+cpsize :
	**/

	let access_flags = (buf[cpsize + 10] as i16) << 8 + (buf[cpsize + 11] as i16)

	let this_class = (buf[cpsize + 12] as i16) << 8 + (buf[cpsize + 13] as i16);
	let super_class = (buf[cpsize + 14] as i16) << 8 + (buf[cpsize + 15] as i16);
	let interfaces_size = (buf[cpsize + 16] as i16) << 8 + (buf[cpsize + 17] as i16);
	
	let mut interfaces : Vec<u8> = Vec::new();
	for i in (18+cpsize)..(18+cpsize+interfaces_size) {
		interfaces.push(buf[i]);
	}

	let field_count = (buf[20 + cpsize + interfaces_size] as i16) << 8 + (buf[21 + cpsize + interfaces_size] as i16);

	for i in 0..field_count {

	}

	panic!("Not implemented");
}

pub fn read_constant_pool_entry(source : &mut Vec<u8>, index : &mut usize) 
	-> Result<CpInfo, ReadError> {
	let mut local_index = *index;

	if (local_index >= source.len()) {
		return Err((local_index, "Expected constant pool entry but file size too short".to_string()));
	}

	let tag = source[local_index];
	let additional_byte_count = match tag {
		1 => {
			//have to deal with variable-length string
			-1
		},
		3 => 4,
		4 => 4,
		5 => 4,
		6 => 8,
		7 => 8,
		8 => 2,
		9 => 4,
		10 => 4,
		11 => 4,
		12 => 4,
		15 => 3,
		16 => 2,
		18 => 4,
		_ => -1
	};

	if additional_byte_count <= 0 {
		return Err((local_index, format!("Expected constant pool entry tag, but got {}", tag)));
	}
	if source.len() <= additional_byte_count + local_index {
		return Err((local_index, 
			format!("Expected additional bytes for constant pool entry, but file too short")));
	}

	let mut additional_bytes = Vec::new();
	for i in local_index..(local_index+additional_byte_count) {
		additional_bytes.push(source[i]);
	}

	local_index = local_index + additional_byte_count;
	*index = local_index;

	Ok(CpInfo {
		tag : tag,
		additional_bytes : additional_bytes
	})
}

fn read_interface_entry(source : &mut Vec<u8>, index : &mut usize) 
	-> Result<Vec<u8>, ReadError> {
	panic!("Not implemented");
}