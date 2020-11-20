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
	test: Option<String>,
}

pub struct Album {
	pub name: String,
	pub pictures: Vec<Picture>,
}

pub struct Picture {
	pub id: String,
	pub order: i32,
    pub caption: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show(Option<String>),
	AlbumRecieved(Option<Album>),
	LoadPictures,
	GetPicture(String),
	PictureReceived(Option<String>),
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
			log!("LoadPictures");
		
			if let Some(album) = &model.album {
				if let Some(first) = album.pictures.first() {
					orders.send_msg(Msg::GetPicture(first.id.clone()));
				}
			}
		},
		Msg::GetPicture(id) => {
			log!(id);
			orders.skip(); // No need to rerender
			let mut data_opt: Option<String> = None;
			let uri = format!("{0}get-picture?id={1}", API_URI, id);
			orders.perform_cmd(async {
				let request = Request::new(uri)
					.method(Method::Get)
					.header(Header::authorization(get_auth()));
				let result = fetch(request).await;
				data_opt = parse_picture(result).await;
				Msg::PictureReceived(data_opt)
			});
		},
		Msg::PictureReceived(data) => {
			log!("PictureReceived");
			model.test = data;
			//orders.force_render_now();
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
					&album.name
				],
				album.pictures.iter().map(|p| div![
					span![&p.id],
					" - ",
					span![&p.order],
					" - ",
					span![&p.caption],
				])

			],
			None => empty![],
		}
	]
}