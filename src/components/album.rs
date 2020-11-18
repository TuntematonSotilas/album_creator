use seed::{self, prelude::*, *};

use crate::utils::{
	request::get_auth, 
	vars::API_URI, 
	parser::parse_album
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Option<Album>,
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
	Recieved(Option<Album>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(id_url) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
				let mut album_opt: Option<Album> = None;

				if let Some(id) = id_url {
					let uri = format!("{0}get-album-detail?id={1}", API_URI, id);
					let request = Request::new(uri)
						.method(Method::Get)
						.header(Header::authorization(get_auth()));
					let response_res = fetch(request).await;

					if let Ok(response) = response_res {
						if let Ok(resp_ok) = response.check_status() {
							album_opt = parse_album(resp_ok).await;
						}
					}
				}
				Msg::Recieved(album_opt)
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