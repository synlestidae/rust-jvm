type ExceptionTableEntry = (u16,u16,u16,u16);
type InnerClass = (u16,u16,u16,u16);

pub enum Attribute {
	ConstantValue {
		attribute_name_index : u16, 
		constantvalue_index : u16},
	Code {
		attribute_name_index : u16,
		max_stack : u16,
    	max_locals : u16,
    	code : Vec<u8>,
    	exception_table : Vec<ExceptionTableEntry>,
    	exception_table[exception_table_length];
    	attributes : Vec<Attribute>
	},
	StackMapTable {
		attribute_name_index : u16,
		entries : Vec<StackMapFrame>
	},
	InnerClasses_attribute {
    	attribute_name_index : u16,
    	attribute_length : u32,
    	number_of_classes : u16,
    	classes : Vec<(u16, u16, u16, u16)>
	},
	EnclosingMethod {
    	attribute_name_index : u16,
    	attribute_length : u32,
    	class_index : u16,
    	method_index : u16
	},
	Synthetic {
    	attribute_name_index : u16,
    	attribute_length : u32
	},
	Signature {
	    attribute_name_index : u16,
	    attribute_length : u32,
	    signature_index : u16
	},
	SourceFile {
	    attribute_name_index : u16,
	    attribute_length : u32,
	    sourcefile_index : u16
	},
	SourceDebugExtension {
	    attribute_name_index : u16,
	    debug_extension : Vec<u8>
	},
	LineNumberTable {
	    attribute_name_index : u16,
	    line_number_table : Vec<(u16, u16)>
	},
	LocalVariableTable {
	    attribute_name_index : u16,
	    local_variable_table : Vec<(u16, u16, u16, u16, u16)>
	},
	LocalVariableTypeTable {
	    attribute_name_index : u16,
	    local_variable_type_table : Vec<(u16, u16, u16, u16, u16)
	},
	Depecrated {
		attribute_name_index : u16
	},
	RuntimeVisibleAnnotations {
	    attribute_name_index : u16,
	    annotations : Vec<Annotation>
	},
	RuntimeInvisibleAnnotations {
	    attribute_name_index : u16,
	    annotations : Vec<Annotation>
	},
	RuntimeVisibleParameterAnnotations {
    	attribute_name_index : u16,
    	parameter_annotations : Vec<(u16, Vec<Annotation>)>,
	},
	AnnotationDefault_attribute {
    	attribute_name_index : u16,
    	attribute_length : u32,
    	default_value : ElementValue
	},
	BootstrapMethods_attribute {
	    attribute_name_index : u16,
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
    	verification_type_info locals[frame_type - 251];
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
    ObjectVariable {cpool_index : u16},
    UninitializedVariable {offset : u16},
} 

pub struct Annotation {
    type_index : u16,
    num_element_value_pairs : u16,
    element_value_pairs : Vec<(u16, ElementValue)>
}

pub enum ElementValue {
	ConstValueIndex(u16),
	EnumConstValue(u16, u16, u16),
	ClassInfoIndex(u16),
	AnnotationValue(Annotation)
}
    tag : u8,
    union {
        u2 ;

        {   u2 type_name_index;
            u2 const_name_index;
        } enum_const_value;

        u2 class_info_index;

        annotation annotation_value;

        {   u2            num_values;
            element_value values[num_values];
        } array_value;
    } value;
}

ElementValue