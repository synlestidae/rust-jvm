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
	let constants = process_constant_pool_table(&raw_classfile.constant_pool_table); 
	let super_name : String; let this_name : String;

	let pattern_to_match = (&constants[raw_classfile.this_class_index as usize - 1],  
		 &constants[raw_classfile.super_class_index as usize - 1]);

	if let 
		(&Constant::Class {name_index : this_class_name_index}, 
		 &Constant::Class {name_index : super_class_name_index}) 
		= pattern_to_match {

		match (&constants[this_class_name_index as usize - 1], 
			&constants[super_class_name_index as usize - 1]) {
			(&Constant::Utf8(ref this), &Constant::Utf8(ref super_)) => {
				this_name = this.clone();
				super_name = super_.clone();
			},
			_ => return Err("This or super class name did not resolve to Utf-8 string".to_string())
		}
	}else{
		return Err(format!("This or super class constant was not CONSTANT_Class_info: {:?}", 
			pattern_to_match));
	}

	let result = RefinedClassFile {
		major_version : raw_classfile.major_version,
		minor_version : raw_classfile.minor_version,
		constant_pool_table : constants.clone(),

		access_flags : process_access_flags(raw_classfile.access_flags),
		this_class : this_name,
		super_class : super_name,

		interface_table : raw_classfile.interface_table.clone(),
		field_table : process_field_table(&constants, 
			&raw_classfile.field_table),
		method_table : process_method_table(&constants, 
			&raw_classfile.method_table),
		attribute_table : process_attributes(&constants, 
			&raw_classfile.attribute_table),
	};

	Ok(result)
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