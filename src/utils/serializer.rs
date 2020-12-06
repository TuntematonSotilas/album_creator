use crate::{
	components::{
		edit_album,
	},
	models::picture,
};

#[derive(serde::Serialize, Debug)]
pub struct Album {
	frid: String,
	name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Picture {
	pub album_id: String,
	pub order: i32,
	pub data: String,
}


pub fn ser_edit_album(album: edit_album::Album) -> Album {
	Album {
		frid: album.frid,
		name: album.name,
	}
}

pub fn ser_new_picture(picture: picture::Picture) -> Picture {
	Picture {
		album_id: picture.album_id,
		order: picture.order,
		data: picture.data,
	}
}