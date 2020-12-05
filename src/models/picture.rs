#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct Picture {
	pub id: Option<String>,
	pub album_id: String,
	pub order: i32,
	pub data: String,
}
