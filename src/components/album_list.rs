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
	albums: Option<Vec<Album>>,
}

#[derive(serde::Deserialize)]
pub struct Id {
	#[serde(rename = "$oid")]
	oid: String,
}

#[derive(serde::Deserialize)]
pub struct Album {
	#[serde(rename = "_id")]
	id: Id,
    name: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show,
	Received(Option<Vec<Album>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::Show => {
			orders.skip(); // No need to rerender
            orders.perform_cmd(async {
				let uri = format!("{0}get-albums", API_URI);
				let request = Request::new(uri)
                	.method(Method::Get)
					.header(Header::authorization(get_auth()));
				let response_res = fetch(request).await;

				let mut opt: Option<Vec<Album>> = None;

				if let Ok(response) = response_res {
					if let Ok(resp_ok) = response.check_status() {
						let albums_res = resp_ok.json::<Vec<Album>>().await;
						if let Ok(albums) = albums_res {
							opt = Some(albums);
						}
					}
				}	
				Msg::Received(opt)
            });
		},
		Msg::Received(albums) => {
			model.albums = albums;
        }
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_title = style! {
		St::TextAlign => "center",
		St::FontSize => rem(2),
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
	};
	let s_no_albums = style! {
		St::TextAlign => "center",
	};
	let s_albums_list = style! {
		St::Margin => rem(1),
		St::Display => "flex",
		St::FlexFlow => "row wrap",
		St::JustifyContent => "center",
	};
	let s_album = style! {
		St::Margin => rem(1),
		St::Width => rem(6),
		St::Height => rem(8),
		St::Background => "radial-gradient(circle at top right, #c51350 -30%, #363062 100%)",
		St::BorderTopRightRadius => rem(0.3),
		St::BorderBottomRightRadius => rem(0.3),
	};
	
	let s_album_name = style! {
		St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
		St::Color => "white",
		St::FontSize => rem(0.5),
	};
	nodes![
		h1![
			s_title,
			"Ablums"
		],
		match &model.albums {
			Some(albums) => div![
				s_albums_list,
				albums.iter().map(|album| div![
					&s_album,
					attrs! { At::Id	=> album.id.oid },
					span![
						&s_album_name,
						&album.name]
				])
			],
			None => div![
				s_no_albums,
				span!["no albums"]],
		}
	]
}