use classfile_preprocessor::rawprocessor::*;
use classfile::raw::*;
use classfile::classfile::RefinedClassFile;
use classfile::attribute::Attribute;
use classfile::info::Info;
use classfile::access_flags::AccessFlags;
use classfile::constant::Constant;

use classfile_preprocessor::refine_constant::refine_constant;
use classfile_preprocessor::refine_attribute::refine_attribute;

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

fn process_method_table(constants : &Vec<Constant>, table : &Vec<RawInfo>)
	-> Vec<Info> {
	table.iter().map(|ref raw_method_info| refine_info(constants, &raw_method_info)).collect()
}

fn process_field_table(constants : &Vec<Constant>, table : &Vec<RawInfo>) 
	-> Vec<Info> {
	table.iter().map(|ref raw_field_info| refine_info(constants, &raw_field_info)).collect()
}

fn process_attributes(constants : &Vec<Constant>, table : &Vec<RawAttributeInfo>) 
	-> Vec<Attribute> {
	table.iter().map(|ref raw_attribute_info| 
		refine_attribute(constants, raw_attribute_info)).collect()
}

fn refine_info(constants : &Vec<Constant>, raw_info : &RawInfo) -> Info {
	let name_constant : &Constant = &constants[raw_info.name_index as usize];
	let descriptor_constant : &Constant = &constants[raw_info.name_index as usize];

	match (name_constant, descriptor_constant) {
		(&Constant::Utf8(ref name), &Constant::Utf8(ref descriptor)) => Info {
			access_flags : process_access_flags(raw_info.access_flags),
		    name : name.clone(),
		    descriptor : descriptor.clone(),
		    attributes : process_attributes(constants, &raw_info.attributes)
		},
		_ => panic!("Invalid name and descriptor constants for method")
	}
}