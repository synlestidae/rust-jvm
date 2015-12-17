use classfile::raw::*;
use classfile::attribute::*;
use classfile::constant::Constant;
use util::*;
use classfile_preprocessor::rawprocessor::*;

pub fn refine_attribute(constants : &Vec<Constant>, raw_attribute : &RawAttributeInfo) -> Attribute {
	let bytes = &raw_attribute.info;
	let name_index = raw_attribute.attribute_name_index;

	let name : &str;

	if let Constant::Utf8(ref const_name) = constants[name_index as usize - 1] {
		name = const_name;
	}
	else {
		panic!("Attribute type must be name constant");
	}

	match name {
		"ConstantValue" => Attribute::ConstantValue {
			constant_value : constants[read_u16(bytes[0], bytes[1]) as usize - 1].clone()
		},
		"Code" => {
			let max_stack = read_u16(bytes[0], bytes[1]);
			let max_locals = read_u16(bytes[2], bytes[3]);
			let code_length = read_u32(bytes[4], bytes[5], bytes[6], bytes[7]) as usize;

			let mut code = Vec::new();
			for i in 0..code_length as usize {
				code.push(bytes[i + 8]);
			}

			let mut exception_table : Vec<ExceptionTableEntry> = Vec::new();
			let exception_table_length = read_u16(bytes[code_length + 8], 
				bytes[code_length + 9]) as usize;

			for j in 0..exception_table_length {
				let i = 10 + code_length as usize + j*8 ;
				let start_pc = read_u16(bytes[i], bytes[i + 1]);
        		let end_pc = read_u16(bytes[i + 2], bytes[i + 3]);
        		let handler_pc = read_u16(bytes[i + 4], bytes[i + 5]);
        		let catch_type = read_u16(bytes[i + 6], bytes[i + 7]);
			}

			let mut j = 10 + code_length + exception_table_length;
			let attributes_count = read_u16(bytes[j as usize], bytes[j as usize + 1]);
			j += 2;
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
			let number_of_entries = read_u16(bytes[0],bytes[1]);
			Attribute::StackMapTable {
				entries : read_stackmaptable_entries(constants, &mut 2, &bytes)
			}
		},
		"Exceptions" => {
			let number_of_exceptions = read_u16(bytes[0],bytes[1]);
			let mut i = 0;
			let mut exceptions_table = Vec::new();
			while i < number_of_exceptions as usize {
				match &constants[read_u16(bytes[i], bytes[i+1]) as usize] {
					&Constant::Class {
						name_index : ni
					} => {
						if let &Constant::Utf8(ref name) = &constants[ni as usize] {
							exceptions_table.push(name.clone());
						}
						else{
							panic!("Expected string for class type");		
						}
						()
					}
					_ => panic!("Expected class type")
				}
				i += 2;
			}
			Attribute::Exceptions {
				exception_table : exceptions_table
			}
		}
		"InnerClasses" => {
			let number_of_classes = read_u16(bytes[0], bytes[1]);
			let mut classes = Vec::new();
			for i in 0..number_of_classes {
				let j = (6 + i*8) as usize;
				let inner_class_info_index = read_u16(bytes[j], bytes[j+1]);
        		let outer_class_info_index = read_u16(bytes[j+2], bytes[j+3]);
        		let inner_name_index = read_u16(bytes[j+4], bytes[j+5]);
        		let inner_class_access_flags = read_u16(bytes[j+6], bytes[j+7]);;
        		classes.push((inner_class_info_index, outer_class_info_index, 
        			inner_name_index, inner_class_access_flags));
			}
			Attribute::InnerClasses {
				classes : classes
			}
		},
		"EnclosingMethod" => {
			let class_index = read_u16(bytes[0], bytes[1]) as usize;
			let method_index = read_u16(bytes[6], bytes[7]) as usize;
			let class = &constants[class_index - 1];
			let method = &constants[method_index - 1];

			Attribute::EnclosingMethod {
				class : class.clone(),
				method : method.clone()
			}
		},
		"Synthetic" => {
			Attribute::Synthetic
		},
		"Signature" => {
			let signature_index = read_u16(bytes[0], bytes[1]);
			match &constants[signature_index as usize - 1] {
				&Constant::Utf8(ref sig_string) => Attribute::Signature {
					signature : sig_string.clone()
				},
				invalidConst => panic!("Unknown signature type: {:?}", 
					invalidConst)
			}
		},
		"SourceFile" => {
			let sourcefile_index = read_u16(bytes[0], bytes[1]);
			match &constants[sourcefile_index as usize - 1] {
				&Constant::Utf8(ref source_string) => Attribute::SourceFile {
					sourcefile : source_string.clone()
				},
				invalidConst => panic!("Unknown sourcefile type: {:?}", invalidConst)
			}
		},
		"SourceDebugExtension" => {
			Attribute::SourceDebugExtension {
				debug_extension : bytes.clone()
			} 
		},
		"LineNumberTable" => {
			let length_of_table = read_u16(bytes[0], bytes[1]) as usize;
			let mut i = 2;
			let mut line_number_info = Vec::new();

			while i < length_of_table {
				let start_pc = read_u16(bytes[i], bytes[i + 1]);
				let line_number = read_u16(bytes[i + 2], bytes[i + 3]);
				line_number_info.push((start_pc, line_number));
				i += 4;
			}

			Attribute::LineNumberTable {
				line_number_table : line_number_info
			}
		},
		"LocalVariableTable" => {
			let length_of_table = read_u16(bytes[0], bytes[1]) as usize;

			let mut i = 2;
			let mut local_variable_table = Vec::new();

			while i < length_of_table {
				let start_pc = read_u16(bytes[i], bytes[i + 1]);
				let length = read_u16(bytes[i + 2], bytes[i + 3]);
				let name_index = read_u16(bytes[i + 4], bytes[i + 5]);
				let descriptor_index = read_u16(bytes[i+ 6], bytes[i + 7]);
				let index = read_u16(bytes[i + 8], bytes[i + 9]);
				local_variable_table.push((start_pc, length, name_index, 
					descriptor_index, index));
			}

			Attribute::LocalVariableTable {
				local_variable_table : local_variable_table
			}
		},
		"LocalVariableTypeTable" => {
			let length_of_table = read_u16(bytes[0], bytes[1]) as usize;

			let mut i = 2;
			let mut local_variable_type_table = Vec::new();

			while i < length_of_table {
				let start_pc = read_u16(bytes[i], bytes[i + 1]);
				let length = read_u16(bytes[i + 2], bytes[i + 3]);
				let name_index = read_u16(bytes[i + 4], bytes[i + 5]);
				let signature_index = read_u16(bytes[i+ 6], bytes[i + 7]);
				let index = read_u16(bytes[i + 8], bytes[i + 9]);
				local_variable_type_table.push((start_pc, length, name_index, 
					signature_index, index));
			}

			Attribute::LocalVariableTypeTable {
				local_variable_type_table : local_variable_type_table
			}
		},
		"Deprecated" => { Attribute::Deprecated },
		"RuntimeVisibleAnnotations" => {panic!("Not implemented");},
		"RuntimeInvisibleAnnotations" => {panic!("Not implemented");},
		"RuntimeVisibleParameterAnnotations" => {panic!("Not implemented");},
		"RuntimeInvisibleParameterAnnotations" => {panic!("Not implemented");},
		"AnnotationDefault" => {panic!("Not implemented");},
		"BootstrapMethods" => {panic!("Not implemented");},
		_ => panic!("Unknown or unsupported attribute name: {}", name)
	}
}

fn read_stackmaptable_entries(constants : &Vec<Constant>, index : &mut usize, bytes : &Vec<u8>) 
	-> Vec<StackMapFrame> {	
	let mut entries = Vec::new();
	while *index < bytes.len() {
		entries.push(read_stackmaptable_entry(constants, index, bytes));
	}
	entries
}

fn read_stackmaptable_entry(constants : &Vec<Constant>, index : &mut usize, bytes : &Vec<u8>) 
	-> StackMapFrame {
	let frame_type = bytes[0];
	*index += 1;

	if (0 <= frame_type && frame_type <= 63) {
		return StackMapFrame::SameFrame
	}
	else if (64 <= frame_type && frame_type <= 127) {
		return StackMapFrame::SameLocals1StackItemFrame {
			stack : read_verification_type_info(constants, index, 1, bytes)}
	}
	else if (frame_type == 247) {
		let offset_delta = read_u16(bytes[*index], bytes[*index + 1]);
		*index += 2;
		return StackMapFrame::SameLocals1StackItemFrameExtended { 
			offset_delta : offset_delta,
			stack : read_verification_type_info(constants, index, 1, bytes)
		}
	}

	panic!("Not implemented");
}

fn read_verification_type_info(constants : &Vec<Constant>, index : &mut usize, count : u16, bytes : &Vec<u8>) 
	-> VerificationTypeInfo {
	*index += 1;
	match bytes[*index - 1] {
		0 => VerificationTypeInfo::TopVariable,
		1 => VerificationTypeInfo::IntegerVariable,
		2 => VerificationTypeInfo::FloatVariable,
		3 => VerificationTypeInfo::DoubleVariable,
		4 => VerificationTypeInfo::LongVariable,
		5 => VerificationTypeInfo::DoubleVariable,
		6 => VerificationTypeInfo::UninitializedThisVariable,
		7 => {
			let cpool_index = read_u16(bytes[*index], bytes[*index + 1]) 
				as usize;
			VerificationTypeInfo::ObjectVariable {
				cpool_object : constants[cpool_index - 1].clone()
			}
		},
		8 => {
			let offset = read_u16(bytes[*index], bytes[*index + 1]);
			*index += 2;
			VerificationTypeInfo::UninitializedVariable {
				offset : offset
			}
		},
		invalid_tag => panic!("Invalid verification type info tag {} at {}", 
			invalid_tag, (*index - 1))
	}

}