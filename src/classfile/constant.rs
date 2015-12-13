pub enum Constant {
	Class {
		name_index : u16
	},
	FieldRef(Ref),
	MethodRef(Ref),
	InterfaceMethodRef(Ref),
	String {
		string_index : u16
	},
	Integer {
		bytes : u32
	},
	Float {
		bytes : u32
	},
	Long {
		high_bytes : u32,
		low_bytes : u32
	},
	Double{
		high_bytes : u32,
		low_bytes : u32,
	},
	NameAndType {
		name_index : u16,
		descriptor_index : u16
	},
	Utf8(String),
	MethodHandle {
		reference_kind : ReferenceKind,
		reference_index : u16
	},
	MethodType {
		descriptor_index : u16
	},
	InvokeDynamic {
		bootstrap_method_attr_index : u16,
    	name_and_type_index : u16
	}
}

pub struct Ref {
	pub class_index : u16,
	pub name_and_type_index : u16
}

pub enum ReferenceKind {
	GetField,
	GetStatic,
	PutField,
	PutStatic,
	InvokeVirtual,
	InvokeDynamic,
	InvokeSpecial,
	NewInvokeSpecial,
	InvokeInterface
}