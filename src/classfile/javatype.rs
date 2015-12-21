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

impl JavaType {
	pub fn size(self : &Self) -> usize {
		match self {
			&JavaType::Reference(_) => 8,
			&JavaType::Primitive(Primitive::Boolean) => 4,
			&JavaType::Primitive(Primitive::ReturnAddress) => 8,
			&JavaType::Primitive(Primitive::Numeric(ref num)) => match num {
				&Numeric::Integral(Integral::Byte) => 1,
				&Numeric::Integral(Integral::Short) => 2,
				&Numeric::Integral(Integral::Int) => 4,
				&Numeric::Integral(Integral::Long) => 8,
				&Numeric::Integral(Integral::Char) => 2,
				&Numeric::FloatingPoint(FloatingPoint::Float) => 4,
				&Numeric::FloatingPoint(FloatingPoint::Double) => 8
			}
		}
	}
}