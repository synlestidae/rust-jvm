use instruction::*;
use instruction_reader::*;

#[test]
fn read_instruction_parses_aload() {
	assert_eq!(read_instruction(&[0x19, 0x01], 
		&mut 0).ok().unwrap(), 
		Instruction::Load(Kind::Ref, 1));
}