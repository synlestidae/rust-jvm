use classfile::raw::*;
use classfile::attribute::*;
use classfile::constant::Constant;
use util::*;
use classfile_preprocessor::rawprocessor::*;

pub fn refine_attribute(constants : &Vec<Constant>, raw_attribute : &RawAttributeInfo) -> Attribute {
	let bytes = &raw_attribute.info;
	let name_index = raw_attribute.attribute_name_index;

	let name : &str;

	if let Constant::Utf8(ref const_name) = constants[name_index as usize] {
		name = const_name;
	}
	else {
		panic!("Attribute type must be name constant");
	}

	match name {
		"ConstantValue" => Attribute::ConstantValue {
			constant_value : constants[read_u16(bytes[4], bytes[5]) as usize].clone()
		},
		"Code" => {
			let max_stack = read_u16(bytes[4], bytes[5]);
			let max_locals = read_u16(bytes[6], bytes[7]);
			let code_length = read_u32(bytes[8], bytes[9], bytes[10], bytes[11]);

			let mut code = Vec::new();
			for i in 12..code_length as usize {
				code.push(bytes[i]);
			}

			let mut exception_table : Vec<ExceptionTableEntry> = Vec::new();
			let exception_table_length = read_u16(bytes[12], bytes[13]);

			let mut j = 12 + code_length;
			while j < 12 + code_length + exception_table_length as u32 {
				let i = j as usize;
				let start_pc = read_u16(bytes[i], bytes[i + 1]);
        		let end_pc = read_u16(bytes[i + 2], bytes[i + 3]);
        		let handler_pc = read_u16(bytes[i + 4], bytes[i + 5]);
        		let catch_type = read_u16(bytes[i + 6], bytes[i + 7]);
        		j = j + 8;
			}


			let attributes_count = read_u16(bytes[j as usize], bytes[j as usize + 1]);

			let attributes = read_attributes_info(bytes, &mut (j as usize), attributes_count as usize)
				.unwrap()
				.iter()
				.map(|attribute| refine_attribute(constants, &attribute))
				.collect();

			Attribute::Code {
				max_stack : max_stack,
    			max_locals : max_locals,
    			code : code,
    			exception_table : exception_table,
    			attributes : attributes
			}
		},
		"StackMapTable" => {
			panic!("Not implemented");
		},
		"Exceptions" => {
			panic!("Not implemented");
		}
		"InnerClasses" => {
			panic!("Not implemented");
		},
		"EnclosingMethod" => {
			panic!("Not implemented");
		},
		"Synthetic" => {
			panic!("Not implemented");
		},
		"Signature" => {
			panic!("Not implemented");
		},
		"SourceFile" => {panic!("Not implemented");},
		"SourceDebugExtension" => {panic!("Not implemented");},
		"LineNumberTable" => {panic!("Not implemented");},
		"LocalVariableTable" => {panic!("Not implemented");},
		"LocalVariableTypeTable" => {panic!("Not implemented");},
		"Deprecated" => {panic!("Not implemented");},
		"RuntimeVisibleAnnotations" => {panic!("Not implemented");},
		"RuntimeInvisibleAnnotations" => {panic!("Not implemented");},
		"RuntimeVisibleParameterAnnotations" => {panic!("Not implemented");},
		"RuntimeInvisibleParameterAnnotations" => {panic!("Not implemented");},
		"AnnotationDefault" => {panic!("Not implemented");},
		"BootstrapMethods" => {panic!("Not implemented");},
		_ => panic!("Unknown or unsupported attribute name: {}", name)
	}

}