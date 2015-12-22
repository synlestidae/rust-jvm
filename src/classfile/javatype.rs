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
			&JavaType::Primitive(Primitive::Numeric(ref num)) => num.size()
		}
	}

	pub fn from(type_name : &str) -> Option<JavaType> {
		let mut i = 0;
		let mut java_type : JavaType;

		return Some(match type_name {
			"int" => JavaType::Primitive(Primitive::Numeric(Numeric::Integral(Integral::Int))),
			"byte" => JavaType::Primitive(Primitive::Numeric(Numeric::Integral(Integral::Byte))),
			"long" => JavaType::Primitive(Primitive::Numeric(Numeric::Integral(Integral::Long))),
			"short" => JavaType::Primitive(Primitive::Numeric(Numeric::Integral(Integral::Short))),
			"char" => JavaType::Primitive(Primitive::Numeric(Numeric::Integral(Integral::Char))),
			"float" => JavaType::Primitive(Primitive::Numeric(Numeric::FloatingPoint(FloatingPoint::Float))),
			"double" => JavaType::Primitive(Primitive::Numeric(Numeric::FloatingPoint(FloatingPoint::Double))),
			_ => {
				JavaType::Reference(Reference::Class(type_name.to_string()))
			}
		});
	}
}

impl Numeric {
	pub fn size(self : &Self) -> usize {
		match self {
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