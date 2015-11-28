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

/*impl PartialEq for Data {
	fn eq(&self, other : &Self) -> bool {
		match (self, other) {
			(&Data::Ref(r1), &Data::Ref(r2)) => r1 == r2,
			(&Data::Float(f1), &Data::Float(f2)) => f1.integer_decode() == f2.integer_decode(),
			(&Data::Int(i1), &Data::Int(i2)) => i1 == i2,
			_ => false
		}
	}

	fn ne(&self, other : &Self) -> bool{
		!(self == other) 
	}
}

impl Eq for Data {}*/

type Index = u8;
type ArrayIndex = usize;
