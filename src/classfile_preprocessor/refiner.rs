use classfile_preprocessor::rawprocessor::*;
use classfile::raw::*;
use classfile::classfile::RefinedClassFile;
use classfile::attribute::Attribute;
use classfile::field_info::FieldInfo;
use classfile::method_info::MethodInfo;
use classfile::access_flags::AccessFlags;
use classfile::constant::Constant;

use classfile_preprocessor::refine_constant::refine_constant;

pub type ClassFileProcessingError = String;

pub fn refine_classfile(raw_classfile : &RawClassFile) -> Result<RefinedClassFile, 
		ClassFileProcessingError> {
	panic!("Not implemented");
}

pub fn process_access_flags(raw_flags : u16) -> AccessFlags {
	AccessFlags::make_flag(raw_flags)
}

fn process_constant_pool_table(table : &Vec<RawCpInfo>) -> Vec<Constant> {
	table.iter().map(refine_constant).collect()
}

fn process_method_table(constants : &Vec<Constant>, table : &Vec<RawMethodInfo>)
	-> Vec<MethodInfo> {
	table.iter().map(refine_method_info).collect()
}

fn process_field_table(constants : &Vec<Constant>, table : &Vec<RawFieldInfo>) 
	-> Vec<FieldInfo> {
	panic!("Not implemented");	
}

fn process_attributes(constants : &Vec<Constant>, table : &Vec<RawAttributeInfo>) 
	-> Vec<Attribute> {
	panic!("Not implemented");
}

fn refine_method_info(constants : &Vec<Constant>, raw_method_info : &RawMethodInfo) -> MethodInfo {
	let name_constant = constants[raw_method_info.name_index as usize];
	let descriptor_constant = constants[raw_method_info.name_index as usize];

	match (name_constant, descriptor_constant) {
		(Constant::Utf8(name), Constant::Utf8(descriptor)) => MethodInfo {
			access_flags : process_access_flags(raw_method_info.access_flags),
		    name : name,
		    descriptor : descriptor,
		    attributes : process_attributes(constants, &raw_method_info.attributes)
		},
		_ => panic!("Invalid name and descriptor constants for method")
	}
}