use bytecode::instruction::*;

pub fn read_instruction(input: &[u8], index_in: &mut usize) -> Result<Instruction, ReadError> {

    let index: usize = *index_in;

    // Do the first check on the index
    if index >= input.len() {
        return Err(ReadError::IndexOutOfBounds(index, input.len()));
    }

    // Begin to match load bytecodes
    match input[0] {
        // aload
        0x19 => {
            let from_where = input[index + 1];
            *index_in = index + 2;
            return Ok(Instruction::Load(Kind::Ref, from_where));
        }
        // dload
        0x18 => {
            let from_where = input[index + 1];
            *index_in = index + 2;
            return Ok(Instruction::Load(Kind::Num(NumKind::Double), from_where));
        }
        // fload
        0x17 => {
            let from_where = input[index + 1];
            *index_in = index + 2;
            return Ok(Instruction::Load(Kind::Num(NumKind::Float), from_where));
        }
        // iload
        0x15 => {
            let from_where = input[index + 1];
            *index_in = index + 2;
            return Ok(Instruction::Load(Kind::Num(NumKind::Int), from_where));
        }
        // lload
        0x16 => {
            let from_where = input[index + 1];
            *index_in = index + 2;
            return Ok(Instruction::Load(Kind::Num(NumKind::Long), from_where));
        }
        _ => {}
    }

    // aload_0..aload_3
    if input[0] <= 0x2d && input[0] >= 0x2a {
        let from_where = input[0] - 0x2a;
        *index_in = index + 1;
        return Ok(Instruction::Load(Kind::Ref, from_where));
    }

    // dload_0..dload_3
    if input[0] >= 0x26 && input[0] <= 0x29 {
        let from_where = input[0] - 0x26;
        *index_in = index + 1;
        return Ok(Instruction::Load(Kind::Num(NumKind::Double), from_where));
    }

    // fload_0..fload_3
    if input[0] >= 0x22 && input[0] <= 0x25 {
        let from_where = input[0] - 0x22;
        *index_in = index + 1;
        return Ok(Instruction::Load(Kind::Num(NumKind::Float), from_where));
    }

    // fload_0..fload_3
    if input[0] >= 0x1a && input[0] <= 0x1d {
        let from_where = input[0] - 0x1a;
        *index_in = index + 1;
        return Ok(Instruction::Load(Kind::Num(NumKind::Int), from_where));
    }

    // lload_0..lload_3
    if input[0] >= 0x1e && input[0] <= 0x21 {
        let from_where = input[0] - 0x1e;
        *index_in = index + 1;
        return Ok(Instruction::Load(Kind::Num(NumKind::Long), from_where));
    }

    // End match load bytecodes

    return Err(ReadError::NotImplemented("Opcode not implemented".to_string()));
}


fn read_32bit_integer(input: &[u8], index: usize) -> Result<u32, ReadError> {
    if input.len() >= 4 {
        return Ok(((input[3] as u32) << 24) + ((input[2] as u32) << 16) +
                  ((input[1] as u32) << 8) + input[0] as u32);
    }
    Err(ReadError::IndexOutOfBounds(index, input.len()))
}

pub enum ReadError {
    IndexOutOfBounds(usize, usize),
    NotImplemented(String),
}
