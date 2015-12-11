use bytecode::instruction::*;
use bytecode_preprocessor::*;

#[test]
fn read_instruction_parses_aload() {
	assert_eq!(read_instruction(&[0x19, 0x01], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 1));

	assert_eq!(read_instruction(&[0x2a], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 0));


	assert_eq!(read_instruction(&[0x2b], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 1));


	assert_eq!(read_instruction(&[0x2c], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 2));


	assert_eq!(read_instruction(&[0x2d], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 3));
}

#[test]
fn read_instruction_parses_iload() {
	assert_eq!(read_instruction(&[0x15, 0x10], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Int, 16));

	assert_eq!(read_instruction(&[0x1a], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Int, 0));


	assert_eq!(read_instruction(&[0x1b], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Int, 1));


	assert_eq!(read_instruction(&[0x1c], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Int, 2));


	assert_eq!(read_instruction(&[0x1d], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Int, 3));
}

#[test]
fn read_instruction_parses_lload() {
	assert_eq!(read_instruction(&[0x16, 0x20], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Long, 32));

	assert_eq!(read_instruction(&[0x1e], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Long, 0));


	assert_eq!(read_instruction(&[0x1f], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Long, 1));


	assert_eq!(read_instruction(&[0x20], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Long, 2));


	assert_eq!(read_instruction(&[0x21], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Long, 3));
}

#[test]
fn read_instruction_parses_dload() {
	assert_eq!(read_instruction(&[0x18, 0x23], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Double, 35));

	assert_eq!(read_instruction(&[0x26], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Double, 0));


	assert_eq!(read_instruction(&[0x27], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Double, 1));


	assert_eq!(read_instruction(&[0x28], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Double, 2));


	assert_eq!(read_instruction(&[0x29], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Double, 3));
}

#[test]
fn read_instruction_parses_fload() {
	assert_eq!(read_instruction(&[0x17, 0x21], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Float, 33));

	assert_eq!(read_instruction(&[0x22], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Float, 0));


	assert_eq!(read_instruction(&[0x23], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Float, 1));


	assert_eq!(read_instruction(&[0x24], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Float, 2));


	assert_eq!(read_instruction(&[0x25], 
			&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Float, 3));
}