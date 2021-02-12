use crate::models::picture;
	
#[derive(Default, Clone)]
pub struct Album {
	pub frid: String,
	pub name: String,
	pub pictures : Vec<picture::Picture>,
}