use classfile::class::Class;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum JavaType {
    Byte,
    Short,
    Char,
    Int,
    Float,
    Double,
    Long,
    Ref(String),
    ArrayRef(Box<JavaType>, u8),
}

impl JavaType {
    pub fn size(self: &Self) -> usize {
        match self {
            &JavaType::Ref(_) => 8,
            &JavaType::ArrayRef(_, _) => 8,
            &JavaType::Byte => 1,
            &JavaType::Short => 2,
            &JavaType::Int => 4,
            &JavaType::Long => 8,
            &JavaType::Char => 2,
            &JavaType::Float => 4,
            &JavaType::Double => 8,
        }
    }

    pub fn parse_field_type(descriptor : &str) -> Result<JavaType, String> {
        return Ok(try!(
            JavaType::parse_field_type_partial(&descriptor
                .chars()
                .collect::<Vec<char>>(), &mut 0)).0);
    }

    pub fn parse_field_type_partial(line : &[char], start_index : &mut usize) -> Result<(JavaType, usize), String> {
        let mut i = *start_index;
        let t = match line[i] {
            'B' => JavaType::Byte,
            'C' => JavaType::Char,
            'D' => JavaType::Double,
            'F' => JavaType::Float,
            'I' => JavaType::Int,
            'J' => JavaType::Long,
            'L' => panic!("Not ready for this"),
            'S' => JavaType::Short,
            'Z' => JavaType::Int,
            '[' => panic!("Not ready for this"),
            _ => return Err(format!("Unexpected character '{}' at index {}", i, line[i])),
        };
        i += 1;
        *start_index = i;
        Ok((t, i))
    }

    /*pub fn from(type_name: &str) -> Option<JavaType> {
        if type_name.len() == 0 {
            None
        } else {
            return Some(match type_name {
                "int" => JavaType::Int,
                "byte" => JavaType::Byte,
                "long" => JavaType::Long,
                "short" => JavaType::Short,
                "char" => JavaType::Char,
                "float" => JavaType::Float,
                "double" => JavaType::Double,
                _ => {
                    let mut i: usize = 0;
                    for c in type_name.chars() {
                        if c != '[' {
                            break;
                        }
                        i += 1;
                    }
                    if i > 0 {
                        let 
                        JavaType::ArrayRef(type_name[i..type_name.len()].to_string(), i as u8)
                    } else {
                        JavaType::Ref(type_name.to_string())
                    }
                }
            });
        }
    }*/
}
