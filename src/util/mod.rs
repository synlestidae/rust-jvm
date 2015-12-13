use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use bit_vec::BitVec;
use std::char;
pub type ReadError = (usize, String);

pub fn read_u16(b1 : u8, b2 : u8) -> u16 {
	return Cursor::new(vec![b1,b2]).read_u16::<BigEndian>().unwrap();
}

pub fn read_u32(b1 : u8, b2 : u8, b3 : u8, b4 : u8) -> u32 {
	return Cursor::new(vec![b1,b2,b3,b4]).read_u32::<BigEndian>().unwrap();
}

pub fn read_utf_string(buf : &Vec<u8>, index : &mut usize, length : usize) 
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

fn read_single(bit_vec : &BitVec, index : &mut usize) -> u32 {
	let local_index = *index;

	let x = ((toNum(bit_vec[local_index + 0]) << 6) + (toNum(bit_vec[local_index + 1]) << 5) + 
			(toNum(bit_vec[local_index + 2]) << 4) + (toNum(bit_vec[local_index + 3]) << 3) + 
			(toNum(bit_vec[local_index + 4]) << 2) + (toNum(bit_vec[local_index + 5]) << 1) + 
			(toNum(bit_vec[local_index + 6]))) as u32;

	*index += 1;

	x
}