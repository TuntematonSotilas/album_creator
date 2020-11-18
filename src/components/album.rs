use seed::{self, prelude::*, *};

use crate::utils::{
	vars::API_URI,
	request::get_auth
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Option<Album>,
}

#[derive(serde::Deserialize)]
pub struct Album {
	info: Info,
	pictures: Vec<Pictures>,
}

#[derive(serde::Deserialize)]
pub struct Info {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct Pictures {
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

// ------------
//    Update
// ------------

pub enum Msg {
	Show(Option<String>),
	Recieved(Option<Album>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(id_url) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
				let mut opt: Option<Album> = None;

				if let Some(id) = id_url {
					let uri = format!("{0}get-album-detail?id={1}", API_URI, id);
					let request = Request::new(uri)
						.method(Method::Get)
						.header(Header::authorization(get_auth()));
					let response_res = fetch(request).await;

					if let Ok(response) = response_res {
						if let Ok(resp_ok) = response.check_status() {
							let albums_res = resp_ok.json::<Album>().await;
							if let Ok(albums) = albums_res {
								opt = Some(albums);
							}
						}
					}
				}
				Msg::Recieved(opt)
			});
		},
		Msg::Recieved(opt) => {
			model.album = opt;
		},
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_album = style! {
		St::Margin => rem(1),
		St::Display => "flex",
		St::FlexDirection => "column",
		St::JustifyContent => "center",
	};
	let s_title = style! {
		St::TextAlign => "center",
		St::FontSize => rem(2),
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
	};
	nodes![
		match &model.album {
			Some(album) => div![
				s_album,
				h1![
					s_title,
					&album.info.name
				],
				album.pictures.iter().map(|picture| div![
					span![&picture.id.value],
					" - ",
					span![&picture.order.value],
					" - ",
					span![&picture.caption],
				])

			],
			None => empty![],
		}
	]
}