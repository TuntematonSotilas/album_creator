use seed::{self, prelude::*};

use crate::{
	components::{
		album_list,
	},
	models,
};

#[derive(serde::Deserialize, Debug)]
pub struct Album {
	info: AlbumInfo,
	pictures: Vec<Picture>,
}

#[derive(serde::Deserialize, Debug)]
pub struct AlbumInfo {
	frid: String,
	name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Picture {
	#[serde(rename = "_id")]	
	id: Id,
	order: Order,
    caption: Option<String>,
}

#[derive(serde::Deserialize, Debug)]	
pub struct Id {	
	#[serde(rename = "$oid")]	
	value: String,	
}

#[derive(serde::Deserialize, Debug)]	
pub struct Order {	
	#[serde(rename = "$numberInt")]	
	value: String,	
}

#[derive(serde::Deserialize, Debug)]
pub struct PictureData {
	data: String,
}

#[derive(serde::Deserialize)]
pub struct PicInserted {
	id: String,
}

pub async fn deser_album_list(result: Result<Response, FetchError>) -> Option<Vec<album_list::Album>> {
	let mut album_opt: Option<Vec<album_list::Album>> = None;
	if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let albums_res = resp_ok.json::<Vec<AlbumInfo>>().await;
			if let Ok(albums) = albums_res {
				let list = albums.into_iter().map(|a|
					album_list::Album {
						frid: a.frid,
						name: a.name,
					}).collect();
				album_opt = Some(list);
			}
		}
	}
	album_opt
}

pub async fn deser_album_det(result: Result<Response, FetchError>) -> Option<models::album::Album> {
	let mut album_opt: Option<models::album::Album> = None;
	if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let album_res = resp_ok.json::<Album>().await;
			if let Ok(album) = album_res {
				let frid = album.info.frid;
				let mut album = models::album::Album {
					frid: frid.clone(),
					name: album.info.name,
					pictures: album.pictures.into_iter().map(|p|
						models::picture::Picture {
							id: Some(p.id.value),
							order: p.order.value.parse().unwrap_or(0),
							caption: p.caption,
							data: None,
							dom: false,
							album_id: frid.clone(),
							saved: true,
						}
					)
					.collect()
				};
				album.pictures.sort_by(|a, b| b.order.cmp(&a.order));
				album.pictures.reverse();
				album_opt = Some(album);
			}
		}
	}
    album_opt
}

pub async fn deser_picture(result: Result<Response, FetchError>) -> Option<String> {
	let mut data: Option<String> = None;
    if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let picture_res = resp_ok.json::<PictureData>().await;
			if let Ok(pic_data) = picture_res {
				data = Some(pic_data.data)
			}
		}
	}
    data
}

pub async fn deser_upload_resp(result: Result<Response, FetchError>) -> Option<String> {
	let mut id: Option<String> = None;
    if let Ok(response) = result {
		if let Ok(resp_ok) = response.check_status() {
			let inserted_res = resp_ok.json::<PicInserted>().await;
			if let Ok(inserted) = inserted_res {
				id = Some(inserted.id)
			}
		}
	}
    id
}