use std::io::Read;
use classfile::raw::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use bit_vec::BitVec;
use std::char;

pub type ReadError = (usize, String);

pub fn read_class_file(source : &mut Read) -> Result<RawClassFile, ReadError> {
	//println!("Reading into buf");
	let mut buf = Vec::new();
	source.read_to_end(&mut buf);

	if buf.len() < 4 {
		return Err((buf.len() - 1, "Expected CAFEBABE magic number. Class file too short.".to_string()))
	}

	if &buf[0..4] != &[0xca,0xfe,0xba,0xbe] {
			return Err((0, format!("Expected CAFEBABE magic number. Got {:?}", &buf[0..4])));
	}

	if buf.len() < 8 {
		return Err((buf.len() - 1, "Expected major/minor version. Class file too short.".to_string()))
	}

	//println!("Reading major, minor numbers");
	let minor_version = read_u16(buf[4], buf[5]);
	let major_version = read_u16(buf[6], buf[7]);

	if buf.len() < 10 {
		return Err((buf.len() - 1, "Expected constant pool offset. Class file too short.".to_string()))
	}

	//println!("Reading the constant pool of ");
	//parse the variable-length constant pool
	let constant_pool_count = read_u16(buf[8],buf[9]) - 1;
	//println!("Reading the constant pool of count {}", constant_pool_count);
	let mut index = 10;
	let mut constant_pool : Vec<RawCpInfo> = Vec::new();
	for i in 0..(constant_pool_count) {
		let cp_info_entry = try!(read_constant_pool_entry(&mut buf, &mut index));
		constant_pool.push(cp_info_entry);
	}

	let cpsize = (index) - 10;
	//println!("Size of constant pool: {}", cpsize);
	//println!("Reading the flags and things");
	let access_flags = read_u16(buf[cpsize + 10], buf[cpsize + 11]);
	let this_class = read_u16(buf[cpsize + 12], buf[cpsize + 13]);
	let super_class = read_u16(buf[cpsize + 14], buf[cpsize + 15]);

	let interfaces_size = read_u16(buf[cpsize + 16], buf[cpsize + 17]);
	let mut interfaces : Vec<u8> = Vec::new();
	//println!("Reading the interfaces at {}", (18 + cpsize));
	for i in (18+cpsize)..(18+cpsize+interfaces_size as usize) {
		interfaces.push(buf[i]);
	}
	//println!("Read {} interfaces", interfaces_size);



	//println!("Reading field count at index {}", 18 + cpsize + interfaces_size as usize);
	let field_count = read_u16(buf[18 + cpsize + interfaces_size as usize], 
		buf[19 + cpsize + interfaces_size as usize]);

	index = 20 + cpsize + interfaces_size as usize;

	println!("Reading {} field info entries at index {}", field_count, index);
	let mut field_info_entries = Vec::new();
	let old_index = index;
	for i in 0..field_count {
		let field_info_entry = try!(read_info_entry(&mut buf, &mut index));
		field_info_entries.push(field_info_entry);
	}
	let fsize = index - old_index;

	if index + 2 >= buf.len() {
		return Err((index, "Expected number of entries in method table. Class file too short".to_string()));
	}

	//println!("Reading method count at index {}", index);

	let method_count = read_u16(buf[index], buf[index+1]);
	index += 2;
	//println!("Reading method {} infos at index {}", method_count, index);
	let mut method_info_entries = Vec::new();
	for i in 0..method_count {
		let method_info_entry = try!(read_info_entry(&mut buf, &mut index));
		method_info_entries.push(method_info_entry);
	}

	if index + 1 >= buf.len() {
		return Err((index, "Expected number of attributes. Class file too short".to_string()));
	}

	let attribute_count = read_u16(buf[index], buf[index+1]);
	index += 2;

	//println!("Reading {} attirbutes at index {}", attribute_count, index);

	/*let mut attribute_entries = Vec::new();
	for i in 0..attribute_count {
		let attribute_info_entry = try!(read_info_entry(&mut buf, &mut index));
		method_info_entries.push(attribute_info_entry);
	}*/

	let attribute_entries = try!(read_attributes_info(&buf, &mut index, attribute_count as usize));


	Ok(RawClassFile {
		minor_version : minor_version,
		major_version : major_version,
		constant_pool_table : constant_pool,
		access_flags : access_flags,
		this_class_index : this_class,
		super_class_index : super_class,
		interface_table : interfaces,
		field_table : field_info_entries,
		method_table : method_info_entries,
		attribute_table : attribute_entries
	})
}

pub fn read_constant_pool_entry(source : &mut Vec<u8>, index : &mut usize) 
		-> Result<RawCpInfo, ReadError> {
	let mut local_index = *index;

	if (local_index >= source.len()) {
		return Err((local_index, "Expected constant pool entry but file too short".to_string()));
	}

	let tag = source[local_index];
	let additional_byte_count : usize = match tag {
		1 => {
			//have to deal with variable-length string
			if local_index as usize + 2 >= source.len() {
				return Err((local_index+1, "Expected additional byte count but file too short".to_string()))
			}

			let str_len = read_u16(source[local_index+1], source[local_index+2]);
			if local_index as usize + 2 + str_len as usize >= source.len() {
				return Err((local_index+2, "Expected constant pool UTF-8 string but file too short".to_string()))
			}

			local_index += 3;

			//println!("Trying to read this dang string: {}", local_index);

			let the_string = read_utf_string(source, &mut local_index, str_len as usize);
			
			*index = local_index;

			return Ok(RawCpInfo {
				tag : tag,
				additional_bytes : the_string.unwrap().into_bytes()
			});
		},
		3 => 4,
		4 => 4,
		5 => 8,
		6 => 8,
		7 => 2,
		8 => 2,
		9 => 4,
		10 => 4,
		11 => 4,
		12 => 4,
		15 => 3,
		16 => 2,
		18 => 4,
		_ => return Err((local_index, format!("Expected valid constant pool entry tag, but got {}", tag)))
	};


	//println!("Source len: {} Byte count: {} Local index: {}", source.len(), additional_byte_count, local_index);

	if source.len() <= additional_byte_count + local_index {
		return Err((local_index, 
			format!("Expected additional bytes for constant pool entry, but file too short")));
	}

	let mut additional_bytes = Vec::new();
	for i in (local_index+1)..(local_index+additional_byte_count) {
		additional_bytes.push(source[i]);
	}

	*index = local_index + 1 + additional_byte_count;

	Ok(RawCpInfo {
		tag : tag,
		additional_bytes : additional_bytes
	})
}

fn read_info_entry(source : &mut Vec<u8>, index : &mut usize) 
	-> Result<RawInfo, ReadError> {
	let mut local_index = *index;
	//println!("Begin index at {}", local_index);
	if local_index + 8 >= source.len() {
		return Err((local_index, format!("Not enough bytes for fixed-size field_info items at index {}. File is {} bytes.", 
			local_index, source.len())));
	}

	let access_flags = read_u16(source[local_index], source[local_index+1]);
	let name_index = read_u16(source[local_index+2], source[local_index+3]);
	let descriptor_index = read_u16(source[local_index+4], source[local_index+5]);
	let attributes_count = read_u16(source[local_index+6], source[local_index+7]);

	if local_index + 14 >= source.len() {
		return Err((local_index, "Not enough bytes for attribute_info structure".to_string()))
	}

	local_index = local_index + 8;

	let attributes_info = try!(read_attributes_info(source, &mut local_index, 
		attributes_count as usize));

	*index = local_index;
	//println!("End index at {}", local_index);
	Ok(RawInfo {
		access_flags : access_flags,             
	    name_index : name_index, 
	    descriptor_index : descriptor_index,
	    attributes : attributes_info
	})
}

fn read_attributes_info(source : &Vec<u8>, index : &mut usize, 
	attributes_count : usize) -> Result<Vec<RawAttributeInfo>, ReadError> {
	let mut attributes_info = Vec::new();
	let mut local_index = *index;

	for i in 0..attributes_count {
		let attribute_name_index = read_u16(source[local_index], source[local_index + 1]);
		let attribute_length = read_u32(source[local_index + 2], source[local_index + 3], 
			source[local_index + 4], source[local_index + 5]);

		local_index = local_index + 6;
		if (attribute_length as usize + local_index as usize) > source.len() {
			return Err((local_index+6, format!("Attribute length at index {} is {}. Not enough bytes for attributes.", local_index, attribute_length)))
		}

		let mut info = Vec::new();
		for i in local_index..(local_index + attribute_length as usize) {
			info.push(source[i]);
		}

		local_index = local_index + attribute_length as usize;

		attributes_info.push(RawAttributeInfo {
			attribute_name_index : attribute_name_index,
			info : info
		});
	}
	*index = local_index;
	Ok(attributes_info)
}

fn read_method_entry(source : &mut Vec<u8>, index : &mut usize) 
	-> Result<RawFieldInfo, ReadError> {
	let local_index = *index;
	panic!("TODO");
}

fn read_interface_entry(source : &mut Vec<u8>, index : &mut usize) 
	-> Result<Vec<u8>, ReadError> {
	panic!("Not implemented");	
}

fn read_u16(b1 : u8, b2 : u8) -> u16 {
	return Cursor::new(vec![b1,b2]).read_u16::<BigEndian>().unwrap();
}

fn read_u32(b1 : u8, b2 : u8, b3 : u8, b4 : u8) -> u32 {
	return Cursor::new(vec![b1,b2,b3,b4]).read_u32::<BigEndian>().unwrap();
}

fn read_utf_string(buf : &Vec<u8>, index : &mut usize, length : usize) 
		-> Result<String, ReadError> {
	//println!("The length of utf 8 string is {}", length);
	let mut output_string = String::new();
	while output_string.len() < length {
		let code_point = char::from_u32(try!(read_char(buf, index)));
		output_string.push(code_point.unwrap());
		//println!("Index: {}, Str: {:}", index, output_string);
	}

	Ok(output_string)
}

fn read_char(buf : &Vec<u8>, index : &mut usize) -> Result<u32, ReadError> {
	let bit_vec = BitVec::from_bytes(&buf[*index..(*index+2)]);

	match (bit_vec.get(0), bit_vec.get(1), bit_vec.get(2), 
				bit_vec.get(3)) {
		(Some(false),_,_,_) => {
			*index += 1;
			Ok(buf[*index - 1] as u32)
		},
		(Some(true), Some(true), Some(false), _) => {
			panic!("Not ready for this");
			//println!("Reading low double");	
			Ok(read_low_double(&bit_vec, index))
		},
		(Some(true), Some(true), Some(true), Some(false)) => {
			panic!("Not ready for this");
			//println!("Warning! Processing of supplementary characters WILL fail.");
			Ok(read_high_double(&bit_vec, index))
		},
		(None, None, None, None) => Err((*index, format!("Invalid unicode: there are {} bits but index is {}", bit_vec.len(), *index).to_string())),
		somethingElse => Err((*index, format!("Invalid unicode: bits prefix {:?} does not match modified UTF-8 pattern", somethingElse).to_string()))
	}
}

fn read_single(bit_vec : &BitVec, index : &mut usize) -> u32 {
	let local_index = *index;

	let x = ((toNum(bit_vec[local_index + 0]) << 6) + (toNum(bit_vec[local_index + 1]) << 5) + 
			(toNum(bit_vec[local_index + 2]) << 4) + (toNum(bit_vec[local_index + 3]) << 3) + 
			(toNum(bit_vec[local_index + 4]) << 2) + (toNum(bit_vec[local_index + 5]) << 1) + 
			(toNum(bit_vec[local_index + 6]))) as u32;

	*index += 1;

	x
}

fn read_low_double(bit_vec : &BitVec, index : &mut usize) -> u32 {
	let local_index = *index;

	let x = ((toNum(bit_vec[local_index + 0]) << 6) + (toNum(bit_vec[local_index + 1]) << 5) + 
		(toNum(bit_vec[local_index + 2]) << 4) + (toNum(bit_vec[local_index + 3]) << 3) + 
		(toNum(bit_vec[local_index + 4]) << 2) + (toNum(bit_vec[local_index + 5]) << 1) + 
		(toNum(bit_vec[local_index + 6]))) as u32;

	let y = ((toNum(bit_vec[local_index + 7]) << 6) + (toNum(bit_vec[local_index + 8]) << 5) + 
		(toNum(bit_vec[local_index + 9]) << 4) + (toNum(bit_vec[local_index + 10]) << 3) + 
		(toNum(bit_vec[local_index + 11]) << 2) + (toNum(bit_vec[local_index + 12]) << 1) + 
		(toNum(bit_vec[local_index + 13]))) as u32;

	*index += 2;

	((x & 0x1f) << 6) + (y & 0x3f)
}

fn read_high_double(bit_vec : &BitVec, index : &mut usize) -> u32 {
	let local_index = *index;

	let x = ((toNum(bit_vec[local_index + 0]) << 6) + (toNum(bit_vec[local_index + 1]) << 5) + 
		(toNum(bit_vec[local_index + 2]) << 4) + (toNum(bit_vec[local_index + 3]) << 3) + 
		(toNum(bit_vec[local_index + 4]) << 2) + (toNum(bit_vec[local_index + 5]) << 1) + 
		(toNum(bit_vec[local_index + 6]))) as u32;

	let y = ((toNum(bit_vec[local_index + 7]) << 6) + (toNum(bit_vec[local_index + 8]) << 5) + 
		(toNum(bit_vec[local_index + 9]) << 4) + (toNum(bit_vec[local_index + 10]) << 3) + 
		(toNum(bit_vec[local_index + 11]) << 2) + (toNum(bit_vec[local_index + 12]) << 1) + 
		(toNum(bit_vec[local_index + 13]))) as u32;

	let z = ((toNum(bit_vec[local_index + 14]) << 6) + (toNum(bit_vec[local_index + 15]) << 5) + 
		(toNum(bit_vec[local_index + 16]) << 4) + (toNum(bit_vec[local_index + 17]) << 3) + 
		(toNum(bit_vec[local_index + 18]) << 2) + (toNum(bit_vec[local_index + 19]) << 1) + 
		(toNum(bit_vec[local_index + 20]))) as u32;

	*index += 3;

	((x & 0xf) << 12) + ((y & 0x3f) << 6) + (z & 0x3f)
}

fn toNum(x : bool) -> u8 {
	match x {
		true => 1, false => 0
	}
}