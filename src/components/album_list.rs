use seed::{self, prelude::*, *};

use crate::utils::{
	vars::API_URI,
	request::get_auth,
	deserializer::deser_album_list,
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	albums: Option<Vec<Album>>,
	base_url: Url,
	frid_to_delete: Option<String>,
}

impl Model {
	pub fn new(url: &Url) -> Self {
		Model {
			albums: None,
			base_url: url.to_base_url(),
			frid_to_delete: None
		}
	}
}

pub struct Album {
	pub frid: String,
    pub name: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show,
	Received(Option<Vec<Album>>),
	ShowConfirm(String),
	AskDelete(String),
	Delete,
	DeleteResp(bool),
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
				let result = fetch(request).await;
				let album_opt = deser_album_list(result).await;
				Msg::Received(album_opt)
            });
		},
		Msg::Received(albums) => {
			model.albums = albums;
		},
		Msg::ShowConfirm(_msg) => (),
		Msg::AskDelete(frid) => {
			model.frid_to_delete = Some(frid);
			orders.send_msg(Msg::ShowConfirm("Delete album ?".into()));
		},
		Msg::Delete => {
			orders.skip(); // No need to rerender
			if let Some(frid) = &model.frid_to_delete {	
				let uri = format!("{0}delete-album?id={1}", API_URI, frid);	
				let mut is_ok = false;
				orders.perform_cmd(async move {
					let request = Request::new(uri)
						.method(Method::Delete)
						.header(Header::authorization(get_auth()));
					let result = fetch(request).await;
					if let Ok(response) = result {
						if response.check_status().is_ok() {
							is_ok = true;
						}
					}
					Msg::DeleteResp(is_ok)
				});
			}
		},
		Msg::DeleteResp(is_ok) => {
			if is_ok {
				if let Some(album) = &mut model.albums {
					if let Some(frid) = &model.frid_to_delete {
						let position = album.iter()
							.position(|a| a.frid == *frid);
						if let Some(pos) = position {
							album.remove(pos);
						}	
					}
				}
			}
		},
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
	let s_album_ctn = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
	};
	let s_album = style! {
		St::Margin => rem(1),
		St::Width => rem(6),
		St::Height => rem(8),
		St::Background => "radial-gradient(circle at top right, #d73e73 -30%, #6a639b 100%)",
		St::BorderTopRightRadius => rem(0.3),
		St::BorderBottomRightRadius => rem(0.3),
		St::Display => "flex",
		St::JustifyContent => "center",
	};
	let s_album_name = style! {
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::Color => "white",
		St::FontSize => rem(0.8),
		St::MarginTop => rem(1),
		St::PaddingLeft => rem(0.7),
		St::OverflowWrap => "anywhere",
	};
	let s_album_border = style! {
		St::Position => "absolute",
		St::Width => rem(0.5),
		St::Height => rem(8),
		St::Background => "linear-gradient(to right, rgba(255, 255, 255, 0.2) 0%, rgba(0, 0, 0, 0.5) 150%)",
		St::MarginLeft => rem(-5.5),
	};
	let s_delete = style! {
		St::FontSize => rem(0.8),
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::TextAlign => "center",
		St::MarginTop => rem(-0.7),
	};
	nodes![
		h1![
			s_title,
			"Albums"
		],
		match &model.albums {
			Some(albums) => div![
				s_albums_list,
				albums.iter().map(|album| {
					let frid = album.frid.clone();

					div![
						&s_album_ctn,
						a![
							div![&s_album_border],
							C!("album_list__album"),
							&s_album,
							attrs! { 
								At::Id	=> album.frid,
								At::Href => model.base_url.clone().add_path_part("album").add_path_part(&album.frid), 
							},
							span![
								&s_album_name,
								&album.name
							]
						],
						span![
							C!("delete_link"),
							&s_delete,
							"delete",
							ev(Ev::Click, |_| Msg::AskDelete(frid)),
						],
					]
				})
			],
			None => div![
				s_no_albums,
				span!["loading"]],
		}
	]
}