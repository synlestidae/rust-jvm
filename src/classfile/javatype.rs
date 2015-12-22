use classfile::class::Class;

#[derive(Clone, Debug)]
pub enum JavaType {
	Byte,
	Short,
	Char,
	Int, 
	Float,
	Double, 
	Long, 
	Ref(String),
	ArrayRef(String, u8)
}

impl JavaType {
	pub fn size(self : &Self) -> usize {
		match self {
			&JavaType::Ref(_) => 8,
			&JavaType::ArrayRef(_,_) => 8,
			&JavaType::Byte => 1,
			&JavaType::Short => 2,
			&JavaType::Int => 4,
			&JavaType::Long => 8,
			&JavaType::Char => 2,
			&JavaType::Float => 4,
			&JavaType::Double => 8
		}
	}

	pub fn from(type_name : &str) -> Option<JavaType> {
		if type_name.len() == 0 {
			None
		}
		else {
			return Some(match type_name {
				"int" => JavaType::Int,
				"byte" => JavaType::Byte,
				"long" => JavaType::Long,
				"short" => JavaType::Short,
				"char" => JavaType::Char,
				"float" => JavaType::Float,
				"double" => JavaType::Double,
				_ => {
					let mut i : usize = 0;
					for c in type_name.chars() {
						if c != '[' {
							break;
						}
						i+=1;
					}
					if i > 0 {
						JavaType::ArrayRef(type_name[i..type_name.len()].to_string(), i as u8)
					}
					else {
						JavaType::Ref(type_name.to_string())
					}
				}
			});
		}
	}
}