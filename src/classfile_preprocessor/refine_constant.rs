use classfile::raw::*;
use classfile::constant::*;
use util::*;

pub fn refine_constant(raw_constant: &RawCpInfo) -> Constant {
    let bytes = &raw_constant.additional_bytes;

    println!("Refining: {:?}", raw_constant);

    match raw_constant.tag {
        7 => Constant::Class { name_index: read_u16(bytes[0], bytes[1]) },
        9 => Constant::FieldRef(make_ref(raw_constant)),
        10 => Constant::MethodRef(make_ref(raw_constant)),
        11 => Constant::InterfaceMethodRef(make_ref(raw_constant)),
        8 => Constant::String { string_index: read_u16(bytes[0], bytes[1]) },
        3 => Constant::Integer { bytes: read_u32(bytes[0], bytes[1], bytes[2], bytes[3]) },
        4 => Constant::Float { bytes: read_u32(bytes[0], bytes[1], bytes[2], bytes[3]) },
        5 => {
            Constant::Long {
                low_bytes: read_u32(bytes[0], bytes[1], bytes[2], bytes[3]),
                high_bytes: read_u32(bytes[4], bytes[5], bytes[6], bytes[7]),
            }
        }
        6 => {
            Constant::Double {
                low_bytes: read_u32(bytes[0], bytes[1], bytes[2], bytes[3]),
                high_bytes: read_u32(bytes[4], bytes[5], bytes[6], bytes[7]),
            }
        }
        12 => {
            Constant::NameAndType {
                name_index: read_u16(bytes[0], bytes[1]),
                descriptor_index: read_u16(bytes[2], bytes[3]),
            }
        }
        1 => Constant::Utf8(String::from_utf8(bytes.clone()).unwrap()),
        15 => make_methodhandle(bytes[0], read_u16(bytes[1], bytes[2])),
        16 => Constant::MethodType { descriptor_index: read_u16(bytes[0], bytes[1]) },
        18 => {
            Constant::InvokeDynamic {
                bootstrap_method_attr_index: read_u16(bytes[0], bytes[1]),
                name_and_type_index: read_u16(bytes[2], bytes[3]),
            }
        }
        _ => panic!("Not implemented"),
    }
}

fn make_ref(raw_constant: &RawCpInfo) -> Ref {
    let bytes = &raw_constant.additional_bytes;
    Ref {
        class_index: read_u16(bytes[0], bytes[1]),
        name_and_type_index: read_u16(bytes[2], bytes[3]),
    }
}

fn make_methodhandle(reference_kind: u8, reference_index: u16) -> Constant {
    Constant::MethodHandle {
        reference_index: reference_index,
        reference_kind: match reference_kind {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeDynamic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            _ => {
                panic!("Invalid method handle reference kind was {}",
                       reference_kind)
            }
        },
    }
}
