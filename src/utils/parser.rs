use seed::{self, prelude::*, *};

use crate::components::album;

#[derive(serde::Deserialize)]
pub struct Album {
	info: Info,
	pictures: Vec<Picture>,
}

#[derive(serde::Deserialize)]
pub struct Info {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct Picture {
	#[serde(rename = "_id")]	
	id: Id,
	order: Order,
    caption: String,
}

#[derive(serde::Deserialize)]	
pub struct Id {	
	#[serde(rename = "$oid")]	
	value: String,	
}

#[derive(serde::Deserialize)]	
pub struct Order {	
	#[serde(rename = "$numberInt")]	
	value: String,	
}

#[derive(serde::Deserialize)]
pub struct PictureData {
	data: String,
}

pub async fn parse_album(result: Result<Response, FetchError>) -> Option<album::Album> {
	let mut album_opt: Option<album::Album> = None;
	if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let album_res = resp_ok.json::<Album>().await;
			if let Ok(album) = album_res {
				let mut album = album::Album {
					name: album.info.name,
					pictures: album.pictures.into_iter().map(|p|
						album::Picture {
							id: p.id.value,
							order: p.order.value.parse().unwrap_or(0),
							caption: p.caption
						}
					)
					.collect()
				};
				album.pictures.sort_by(|a, b| b.order.cmp(&a.order));
				album_opt = Some(album);
			}
		}
	}
    album_opt
}

pub async fn parse_picture(result: Result<Response, FetchError>) -> Option<String> {
	let data: Option<String> = None;
    if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let picture_res = resp_ok.json::<PictureData>().await;
			if let Ok(pic_data) = picture_res {
				log!(pic_data.data);
			}
		}
	}
    data
}