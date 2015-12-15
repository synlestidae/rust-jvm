#[derive(Debug, Clone)]
pub struct AccessFlags {
	pub ACC_PUBLIC : bool,
	pub ACC_PRIVATE : bool,
	pub ACC_PROTECTED : bool,
	pub ACC_STATIC : bool,
	pub ACC_FINAL : bool,
	pub ACC_SYNCHRONIZED : bool,
	pub ACC_BRIDGE : bool,
	pub ACC_VARARGS : bool,
	pub ACC_NATIVE : bool,
	pub ACC_ABSTRACT : bool,
	pub ACC_STRICT : bool,
	pub ACC_SYNTHETIC : bool
}

impl AccessFlags {
	pub fn new() -> AccessFlags {
		AccessFlags {
			ACC_PUBLIC: false,
			ACC_PRIVATE: false,
			ACC_PROTECTED: false,
			ACC_STATIC: false,
			ACC_FINAL: false,
			ACC_SYNCHRONIZED: false,
			ACC_BRIDGE: false,
			ACC_VARARGS: false,
			ACC_NATIVE: false,
			ACC_ABSTRACT: false,
			ACC_STRICT: false,
			ACC_SYNTHETIC: false
		}
	}

	pub fn make_flag(flag_value : u16) -> AccessFlags {
			let mut flag = AccessFlags::new();
			if (flag_value & 0x0001 != 0) {
				flag.ACC_PUBLIC = true;
			}
			if (flag_value & 0x0001 != 0) {
				flag.ACC_PUBLIC = true;
			}
			if (flag_value & 0x0002 != 0) {
				flag.ACC_PRIVATE = true;
			}
			if (flag_value & 0x0004 != 0) {
				flag.ACC_PROTECTED = true;
			}
			if (flag_value & 0x0008 != 0) {
				flag.ACC_STATIC = true;
			}
			if (flag_value & 0x0010 != 0) {
				flag.ACC_FINAL = true;
			}
			if (flag_value & 0x0020 != 0) {
				flag.ACC_SYNCHRONIZED = true;
			}
			if (flag_value & 0x0040 != 0) {
				flag.ACC_BRIDGE = true;
			}
			if (flag_value & 0x0080 != 0) {
				flag.ACC_VARARGS = true;
			}
			if (flag_value & 0x0100 != 0) {
				flag.ACC_NATIVE = true;
			}
			if (flag_value & 0x0400 != 0) {
				flag.ACC_ABSTRACT = true;
			}
			if (flag_value & 0x0800 != 0) {
				flag.ACC_STRICT = true;
			}
			if (flag_value & 0x1000 != 0) {
				flag.ACC_SYNTHETIC = true;
			}
			flag
	}
}