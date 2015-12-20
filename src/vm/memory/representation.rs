pub trait Representation {
	fn total_size(self : &Self) -> usize;
	fn private_field_offset_unsafe(self : &Self, field_name : &str) -> usize;
	fn public_field_offset_unsafe(self : &Self, field_name : &str) -> usize;
	fn private_field_offset(self : &Self, field_name : &str) -> Option<usize>;
	fn public_field_offset(self : &Self, field_name : &str) -> Option<usize>;
}