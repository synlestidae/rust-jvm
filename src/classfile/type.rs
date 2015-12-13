use classfile::class::Class;

pub enum Type {
	Primitive(Primitive),
	Reference(Reference)
}

pub enum Primitive {
	Numeric(Numeric),
	Boolean,
	ReturnAddress
}

pub enum Numeric {
	Integral(Integral),
	FloatingPoint(FloatingPoint)
}

pub enum Integral {
	Byte,
	Short,
	Int,
	Long,
	Char
}

pub enum FloatingPoint {
	Float,
	Double
}

pub enum Reference {
	Class(Class),
	Array(Type),
	Interface(Interface),
	Null
}