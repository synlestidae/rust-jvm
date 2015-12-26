use std::cmp::{PartialEq, Eq};
use classfile::constant::Constant;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Instruction {
    // Load and store instructions
    Load(Kind, Index),
    ArrayLoad(Kind, ArrayIndex),
    PushConst(Kind, u64),
    Store(Data, Index),
    Wide,
    // Arithmetic instructiosn
    Add(NumKind),
    Sub(NumKind),
    Mul(NumKind),
    Div(NumKind),
    Rem(NumKind),
    Neg(NumKind),
    Shift(NumKind),
    BitOr(NumKind),
    BitAnd(NumKind),
    BitEx(NumKind),
    Inc(Index, u32),
    Cmp(NumKind),
    Cmpg(NumKind),
    Cmpl(NumKind),
    // Stack manipulation instructions
    Push(Kind),
    Pop,
    Pop2,
    Dup,
    Dup2,
    Dupx1,
    Dupx2,
    Dup2x1,
    Dup2x2,
    Swap,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Ref,
    Num(NumKind),
    Null,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum NumKind {
    Byte,
    Int,
    Float,
    Double,
    Long,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Data {
    Ref(usize),
    Float(i32),
    Double(i64),
    Int(u32),
    Long(i64),
}

type Index = u8;
type ArrayIndex = usize;
