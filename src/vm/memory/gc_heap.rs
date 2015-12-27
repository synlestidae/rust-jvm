use classfile::javatype::*;
use vm::class::Class;
use vm::memory::heap::*;
use vm::memory::representation::*;
use std::iter::repeat;
use std::collections::HashMap;
use vm::LoadedClasses;

pub struct GcHeap {
    eden: Box<[u8]>,
    tenured: Box<[u8]>,

    eden_pointer: usize,
    tenured_pointer: usize,

    virtual_map: HashMap<usize, (HeapKind, usize, JavaType)>,
}

#[derive(Debug, Clone, Copy, Hash)]
enum HeapKind {
    Eden,
    Tenured,
}

impl GcHeap {
    pub fn new(size: usize) -> GcHeap {
        let eden: Vec<u8>;
        let tenured: Vec<u8>;

        if size < 2048 {
            eden = repeat(0).take(2048).collect();
            tenured = repeat(0).take(2048).collect();
        } else {
            eden = repeat(0).take(size / 5).collect();
            tenured = repeat(0).take(size).collect();
        }

        GcHeap {
            eden: eden.into_boxed_slice(),
            tenured: tenured.into_boxed_slice(),

            eden_pointer: 0,
            tenured_pointer: 0,

            virtual_map: HashMap::new(),
        }
    }
}

impl Heap for GcHeap {
    fn allocate_object(self: &mut Self, java_class: &Class) -> Option<usize> {
        let size = java_class.total_size();

        if size > self.tenured.len() || size + self.tenured_pointer >= self.tenured.len() {
            return None;
        }

        if self.eden_pointer + size < self.eden.len() {
            let pointer = self.eden_pointer;
            self.virtual_map.insert(pointer,
                                    (HeapKind::Eden, pointer, JavaType::Ref(java_class.name())));
            self.eden_pointer += size;
            return Some(pointer);
        } else {
            return None;
        }
    }

    fn allocate_type(self: &mut Self, java_type: JavaType) -> Option<usize> {
        let size = java_type.size();

        if size > self.tenured.len() || size + self.tenured_pointer >= self.tenured.len() {
            return None;
        }

        if self.eden_pointer + size < self.eden.len() {
            let pointer = self.eden_pointer;
            self.virtual_map.insert(pointer, (HeapKind::Eden, pointer, java_type));
            self.eden_pointer += size;
            return Some(pointer);
        } else {
            return None;
        }
    }

    fn get_object_mut<'a>(self: &'a mut Self,
                          index: usize,
                          classes: &LoadedClasses)
                          -> Option<&'a mut [u8]> {
        let (heap, lower_index, name) = match &self.virtual_map.get(&index) {
            &Some(&(HeapKind::Eden, mindex, JavaType::Ref(ref name))) => {
                (&mut self.eden, mindex, name)
            }
            &Some(&(HeapKind::Tenured, mindex, JavaType::Ref(ref name))) => {
                (&mut self.tenured, mindex, name)
            }
            _ => return None,
        };

        if let Some(ref class) = classes.find_class(name) {
            let upper_index = lower_index + class.total_size();
            if upper_index < heap.len() {
                return Some(&mut heap[lower_index..upper_index]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn maximum_size(self: &Self) -> usize {
        self.eden.len() + self.tenured.len()
    }

    fn current_size(self: &Self) -> usize {
        self.eden_pointer + self.tenured_pointer
    }

    fn garbage_collect(roots: &[usize], classes: &LoadedClasses) -> usize {
        panic!("Not implemented")
    }

    fn garbage_collect_with_report(roots: &[usize], classes: &LoadedClasses) -> GcReport {
        panic!("Not implemented")
    }
}
