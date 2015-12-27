use vm::memory::representation::*;
use vm::class::*;
use vm::LoadedClasses;
use classfile::javatype::*;


pub trait Heap {
    fn allocate_object(self: &mut Self, class: &Class) -> Option<usize>;
    fn allocate_type(self: &mut Self, java_type: JavaType) -> Option<usize>;
    fn get_object_mut<'a>(self: &'a mut Self,
                          index: usize,
                          classes: &LoadedClasses)
                          -> Option<&'a mut [u8]>;
    fn maximum_size(self: &Self) -> usize;
    fn current_size(self: &Self) -> usize;
    fn garbage_collect(roots: &[usize], classes: &LoadedClasses) -> usize;
    fn garbage_collect_with_report(roots: &[usize], classes: &LoadedClasses) -> GcReport;
}

pub struct GcReport {
    total_bytes_freed: usize,
    total_objects_freed: usize,
}
