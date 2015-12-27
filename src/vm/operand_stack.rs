pub type OperandStack = Vec<Operand>;

pub enum Operand {
    Reference(usize),
    Float(u32),
    Double(u64),
    Int(u32),
    Byte(u8),
}
