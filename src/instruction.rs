use std::cmp::{PartialEq, Eq};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Instruction{
	Load(Kind, Index),
	ArrayLoad(Kind, ArrayIndex),
	Store(Data, Index),
	Push(Kind),
	Pop(Kind)
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Kind {
	Int, Ref, Float, Double, Long
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Data {
	Ref(usize),
	Float(i32),
	Double(i64),
	Int(u32),
	Long(i64)
}

type Index = u8;
type ArrayIndex = usize;
