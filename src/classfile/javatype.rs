use classfile::class::Class;

#[derive(Clone, Debug)]
pub enum JavaType {
	Primitive(Primitive),
	Reference(Reference)
}

#[derive(Clone, Debug)]
pub enum Primitive {
	Numeric(Numeric),
	Boolean,
	ReturnAddress
}

#[derive(Clone, Debug)]
pub enum Numeric {
	Integral(Integral),
	FloatingPoint(FloatingPoint)
}

#[derive(Clone, Debug)]
pub enum Integral {
	Byte,
	Short,
	Int,
	Long,
	Char
}

#[derive(Clone, Debug)]
pub enum FloatingPoint {
	Float,
	Double
}

#[derive(Clone, Debug)]
pub enum Reference {
	Class(String),
	Array(u8, Box<JavaType>),
	Interface(String),
	Null
}