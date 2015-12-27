use vm::Class;

pub struct LoadedClasses {
    classes: Vec<Class>,
}

impl LoadedClasses {
    pub fn new() -> LoadedClasses {
        LoadedClasses { classes: Vec::new() }
    }

    pub fn find_class(self: &Self, class_name: &str) -> Option<Class> {
        for class in self.classes.iter() {
            if &class.name() == class_name {
                return Some(class.clone());
            }
        }
        None
    }

    pub fn add_class(self: &mut Self, class: Class) {
        self.classes.push(class)
    }
}
