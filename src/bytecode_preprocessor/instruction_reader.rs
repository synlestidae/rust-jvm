use bytecode::instruction::*;

pub fn read_instruction(input : &[u8], index_in : &mut usize) 
	-> Result<Instruction, ReadError> {
	
	let index : usize = *index_in;
	
	//Do the first check on the index	
	if index >= input.len() {
		return Err(ReadError::IndexOutOfBounds(index, input.len()))
	}
	/** Zero-parameter instructions **/

	//
	if input[*index_in] == 0x01 {
		*index_in += 1;
		return Ok(Instruction::PushConst(Kind::Null, 0));
	}

	//pop
	if input[*index_in] == 0x57 {
		*index_in += 1;
		return Ok(Instruction::Pop)
	}

	/** Single-parameter instructions **/

	//Check whether at least two bytecodes
	if index >= input.len() {
		return Err(ReadError::IndexOutOfBounds(index, input.len()))
	}

	match input[*index_in] {
		//aload
		0x19 => {
			let from_where = input[index+1];
			*index_in = index + 2;
			return Ok(Instruction::Load(Kind::Ref, from_where));	
		},
		//dload
		0x18 => {
			let from_where = input[index+1];
			*index_in = index + 2;
			return Ok(Instruction::Load(Kind::Double, from_where));
		},
		//fload
		0x17 => {
			let from_where = input[index+1];
			*index_in = index + 2;
			return Ok(Instruction::Load(Kind::Float, from_where));
		},
		//iload
		0x15 => {
			let from_where = input[index+1];
			*index_in = index + 2;
			return Ok(Instruction::Load(Kind::Int, from_where));
		}
		//lload
		0x16 => {
			let from_where = input[index+1];
			*index_in = index + 2;
			return Ok(Instruction::Load(Kind::Long, from_where));
		}
		_ => {}
	}

	//aload_0..aload_3
	if input[*index_in] <= 0x2d && input[*index_in] >= 0x2a {
		let from_where = input[*index_in] - 0x2a;
		*index_in = index + 1;
		return Ok(Instruction::Load(Kind::Ref, from_where));	
	}

	//dload_0..dload_3
	if input[*index_in] >= 0x26 && input[*index_in] <= 0x29 {
		let from_where = input[*index_in] - 0x26;
		*index_in = index + 1;
		return Ok(Instruction::Load(Kind::Double, from_where));	
	}

	//fload_0..fload_3
	if input[*index_in] >= 0x22 && input[*index_in] <= 0x25 {
		let from_where = input[*index_in] - 0x22;
		*index_in = index + 1;
		return Ok(Instruction::Load(Kind::Float, from_where));	
	}

	//fload_0..fload_3
	if input[*index_in] >= 0x1a && input[*index_in] <= 0x1d {
		let from_where = input[*index_in] - 0x1a;
		*index_in = index + 1;
		return Ok(Instruction::Load(Kind::Int, from_where));	
	}

	//lload_0..lload_3
	if input[*index_in] >= 0x1e && input[*index_in] <= 0x21 {
		let from_where = input[*index_in] - 0x1e;
		*index_in = index + 1;
		return Ok(Instruction::Load(Kind::Long, from_where));	
	}

	//bipush
	if input[*index_in] == 0x10 {
		*index_in += 1;
		let byte = input[index_in];
		return Ok(Instruction::PushConst(Kind::NumKind(NumKind::Byte), byte));
	}

	//dconst_0, dconst_1
	if input[*index_in] == 0x0e || input[*index_in] == 0x0f {
		let byte = match input[*index_in] - 0x0e {
			0 => 0x0,
			1 => 0x3f800000,
			_ => panic!("Major internal error occurred. Program reached impossible state.")
		};
		*index_in += 1;
		return Ok(Instruction::PushConst(Kind::NumKind(NumKind::Float), byte));
	}

	//fconst_0..dconst_2
	if 0x0b <= input[*index_in] && input[*index_in] <= 0x0d {
		let byte = match input[*index_in] - 0x0b {
			0 => 0x0,
			1 => 0x3ff0000000000000,
			2 => 0x4000000000000000,
			_ => panic!("Major internal error occurred. Program reached impossible state.")
		};
		*index_in += 1;
		return Ok(Instruction::PushConst(Kind::NumKind(NumKind::Double), byte));
	}

	//lconst_0, lconst_1
	if 0x09 <= input[*index_in] && input[*index_in] <= 0x0a {
		let byte = input[*index_in] - 0x09;
		*index_in += 1;
		return Ok(Instruction::PushConst(Kind::NumKind(NumKind::Double), byte));
	}

	//lconst_0, lconst_1
	if 0x09 <= input[*index_in] && input[*index_in] <= 0x0a {
		let byte = input[*index_in] - 0x09;
		*index_in += 1;
		return Ok(Instruction::PushConst(Kind::NumKind(NumKind::Double), byte));
	}

	/* End match load bytecodes */

	return Err(ReadError::NotImplemented("Opcode not implemented".to_string()));
}

 
fn read_32bit_integer(input : &[u8], index : usize) -> Result<u32, ReadError> {
	if input.len() >= 4 {
		return Ok(((input[3] as u32) << 24) + ((input[2] as u32) << 16) + 
			((input[1] as u32) << 8) + input[*index_in] as u32)
	}
	Err(ReadError::IndexOutOfBounds(index, input.len()))
}

pub enum ReadError {
	IndexOutOfBounds(usize, usize),
	NotImplemented(String)
}