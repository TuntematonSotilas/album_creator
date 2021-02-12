use crate::models::{picture, album};

#[derive(serde::Serialize, Debug)]
pub struct Album {
	frid: String,
	name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Picture {
	pub id: Option<String>,
	pub album_id: String,
	pub order: i32,
	pub data: String,
	pub caption: Option<String>,
}


pub fn ser_edit_album(album: album::Album) -> Album {
	Album {
		frid: album.frid,
		name: album.name,
	}
}

pub fn ser_edit_picture(picture: picture::Picture) -> Picture {
	Picture {
		id: picture.id,
		album_id: picture.album_id,
		order: picture.order,
		data: picture.data.unwrap_or(String::new()),
		caption: picture.caption
	}
}