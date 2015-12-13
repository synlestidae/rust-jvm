use classfile_preprocessor::rawprocessor::*;
use classfile::raw::*;
use classfile::classfile::RefinedClassFile;
use classfile::attribute::Attribute;
use classfile::field_info::FieldInfo;
use classfile::method_info::MethodInfo;
use classfile::access_flags::AccessFlags;
use classfile::constant::Constant;

pub type ClassFileProcessingError = String;

pub fn refine_classfile(raw_classfile : &RawClassFile) -> Result<RefinedClassFile, 
		ClassFileProcessingError> {
	panic!("Not implemented");
}

pub fn process_access_flags(raw_flags : u16) -> AccessFlags {
	AccessFlags::make_flag(raw_flags)
}

fn process_constant_pool_table(table : &Vec<RawCpInfo>) -> Vec<Constant> {
	table.iter().map(|ref cpinfo| Constant::Integer{bytes : 0}).collect()
}

fn process_method_table(table : &Vec<RawMethodInfo>) -> Vec<FieldInfo> {
	panic!("Not implemented");
}

fn process_field_table(table : &Vec<RawFieldInfo>) -> Vec<FieldInfo> {
	panic!("Not implemented");	
}

fn process_attribute_table(table : &Vec<RawAttributeInfo>) -> Vec<Attribute> {
	panic!("Not implemented");
}