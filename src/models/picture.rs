#[derive(Clone, Default)]
pub struct Picture {
	pub id: Option<String>,
	pub album_id: String,
	pub order: i32,
	pub data: Option<String>,
	pub caption: Option<String>,
	pub dom: bool,
}
