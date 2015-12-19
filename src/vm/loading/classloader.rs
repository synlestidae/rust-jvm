use std::path::Path;
use vm::LoadedClasses;
use vm::LoadedClass;
use vm::Package;

pub trait ClassLoader {
	fn load_class(class_name : &str) -> LoadedClass;
	fn define_class(name : &str, bytes : &[u8]) -> LoadedClass;
	fn define_package(name : &str, 
					spec_title : &str,
                    spec_version : &str,
                    spec_vendor : &str,
                    impl_title : &str,
                    impl_version : &str,
                    impl_vendor : &str,
                    seal_base : &str);
	fn get_package(name : &str) -> Package;
	fn get_packages() -> Vec<Package>;
	fn find_library() -> String;
	fn set_default_assertion_status(enabled : bool);
	fn set_package_assertion_status(package_name : &str, enabled : bool);
	fn set_class_assertion_status(class_name : &str, enabled : bool);
	fn clear_assertion_status();
}

pub struct BootstrapClassLoader {
	class_paths : Vec<Box<Path>>,
	loaded_classes : LoadedClasses
}

impl ClassLoader for BootstrapClassLoader {
	fn load_class(class_name : &str) -> LoadedClass {
		panic!("Not implemented")
	}

	fn define_class(name : &str, bytes : &[u8]) -> LoadedClass {
		panic!("Not implemented")
	}

	fn define_package(name : &str, 
					spec_title : &str,
                    spec_version : &str,
                    spec_vendor : &str,
                    impl_title : &str,
                    impl_version : &str,
                    impl_vendor : &str,
                    seal_base : &str) {
		panic!("Not implemented");
	}
	fn get_package(name : &str) -> Package {
		panic!("Not implemented")
	}

	fn get_packages() -> Vec<Package> {
		panic!("Not implemented")
	}

	fn find_library() -> String {
		panic!("Not implemented")
	}

	fn set_default_assertion_status(enabled : bool) {
		panic!("Not implemented")
	}

	fn set_package_assertion_status(package_name : &str, enabled : bool) {
		panic!("Not implemented")
	}

	fn set_class_assertion_status(class_name : &str, enabled : bool) {
		panic!("Not implemented")
	}

	fn clear_assertion_status() {
		panic!("Not implemented")
	}
}