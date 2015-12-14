use classfile::constant::Constant;

pub type ExceptionTableEntry = (u16, u16, u16, Constant);
pub type InnerClass = (u16,u16,u16,u16);

pub enum Attribute {
	ConstantValue {
		constant_value : Constant
	},
	Code {
		max_stack : u16,
    	max_locals : u16,
    	code : Vec<u8>,
    	exception_table : Vec<ExceptionTableEntry>,
    	attributes : Vec<Attribute>
	},
	StackMapTable {
		entries : Vec<StackMapFrame>
	},
	InnerClasses_attribute {
    	classes : Vec<(u16, u16, u16, u16)>
	},
	EnclosingMethod {    	class : Constant,
    	method : Constant
	},
	Synthetic,
	Signature {	    signature : String
	},
	SourceFile {	    sourcefile : String
	},
	SourceDebugExtension {
	    debug_extension : Vec<u8>
	},
	LineNumberTable {
	    line_number_table : Vec<(u16, u16)>
	},
	LocalVariableTable {
	    local_variable_table : Vec<(u16, u16, u16, u16, u16)>
	},
	LocalVariableTypeTable {
	    local_variable_type_table : Vec<(u16, u16, u16, u16, u16)>
	},
	Depecrated {
		attribute_name : String
	},
	RuntimeVisibleAnnotations {
	    annotations : Vec<Annotation>
	},
	RuntimeInvisibleAnnotations {
	    annotations : Vec<Annotation>
	},
	RuntimeVisibleParameterAnnotations {
    	parameter_annotations : Vec<(u16, Vec<Annotation>)>,
	},
	AnnotationDefault_attribute {
    	default_value : ElementValue
	},
	BootstrapMethods_attribute {
	    bootstrap_methods : Vec<(u16, u16, Vec<u16>)>
	}
}

pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {stack : VerificationTypeInfo},
    SameLocals1StackItemFrameExtended {stack : VerificationTypeInfo},
    ChopFrame {offset_delta : u16},
    SameFrameExtended {offset_delta : u16},
    AppendFrame {
    	offset_delta : u16, 
    	locals : Vec<VerificationTypeInfo>
    },
    FullFrame {
	    offset_delta : u16,
	    number_of_locals : u16,
	    locals : Vec<VerificationTypeInfo>,
	    stack : Vec<VerificationTypeInfo>
	}
}

pub enum VerificationTypeInfo {
	TopVariable,
    IntegerVariable,
    FloatVariable,
    LongVariable,
    DoubleVariable,
    NullVariable,
    UninitializedThisVariable,
    ObjectVariable {cpool_object : Constant},
    UninitializedVariable {offset : u16},
} 

pub struct Annotation {
    pub annotation_type : String,
    pub element_value_pairs : Vec<(String, ElementValue)>
}

pub enum ElementValue {
	ConstValueIndex(u16),
	EnumConstValue(u16, u16, u16),
	ClassInfoIndex(u16),
	AnnotationValue(Annotation)
}