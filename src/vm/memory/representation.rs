pub trait Representation {
    fn total_size(self: &Self) -> usize;
    fn field_offset(self: &Self, field_name : &str) -> Option<usize>;
}
