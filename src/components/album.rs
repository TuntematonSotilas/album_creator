use seed::{self, prelude::*, *};

use crate::utils::{
	request::get_auth, 
	vars::API_URI, 
	parser::parse_album, 
	parser::parse_picture,
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Option<Album>,
}

#[derive(Clone)]
pub struct Album {
	pub name: String,
	pub pictures: Vec<Picture>,
}

#[derive(Clone)]
pub struct Picture {
	pub id: String,
	pub order: i32,
	pub caption: String,
	pub data: Option<String>,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show(Option<String>),
	AlbumRecieved(Option<Album>),
	LoadPictures,
	GetPicture(String),
	PictureReceived(Option<String>, String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(id_url) => {
			orders.skip(); // No need to rerender
			let mut album_opt: Option<Album> = None;
			orders.perform_cmd(async {
				if let Some(id) = id_url {
					let uri = format!("{0}get-album-detail?id={1}", API_URI, id);
					let request = Request::new(uri)
						.method(Method::Get)
						.header(Header::authorization(get_auth()));
					let result = fetch(request).await;
					album_opt = parse_album(result).await;
				}
				Msg::AlbumRecieved(album_opt)
			});
		},
		Msg::AlbumRecieved(opt) => {
			model.album = opt;
			orders.send_msg(Msg::LoadPictures);
		},
		Msg::LoadPictures => {
			if let Some(album) = &model.album {
				album.pictures.clone()
					.into_iter()
					.take(10).for_each(|p| {
						orders.send_msg(Msg::GetPicture(p.id));
					});
			}
		},
		Msg::GetPicture(id) => {
			orders.skip(); // No need to rerender
			let uri = format!("{0}get-picture?id={1}", API_URI, id);
			orders.perform_cmd(async {
				let request = Request::new(uri)
					.method(Method::Get)
					.header(Header::authorization(get_auth()));
				let result = fetch(request).await;
				let data_opt = parse_picture(result).await;
				Msg::PictureReceived(data_opt, id)
			});
		},
		Msg::PictureReceived(data, id) => {
			if let Some(album) = &mut model.album {
				album.pictures.iter_mut()
					.find(|p| p.id == id)
					.map(|p| p.data = data);
			}
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
	let s_picture = style! {
		St::MaxWidth => rem(20),
	};
	nodes![
		match &model.album {
			Some(album) => div![
				s_album,
				h1![
					s_title,
					&album.name
				],
				album.pictures.iter().map(|p| div![
					match &p.data {
						Some(d) => img![
							&s_picture,
							attrs!{ At::Src => d }
						],
						_ => empty![]
					},
					span![&p.caption],
				])

			],
			None => empty![],
		}
	]
}