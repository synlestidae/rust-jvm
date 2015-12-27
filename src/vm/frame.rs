use vm::OperandStack;
use vm::RuntimeConstantPool;

pub struct Frame<'a> {
    local_variables: Vec<u16>,
    operand_stack: OperandStack,
    runtime_constant_pool: &'a RuntimeConstantPool,
}
