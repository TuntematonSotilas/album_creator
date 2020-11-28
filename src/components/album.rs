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

const MAX_LOAD: usize = 10;

#[derive(Default)]
pub struct Model {
	album: Option<Album>,
	loaded: usize,
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
	pub dom: bool,
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
			model.album = None;
			model.loaded = 0;
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
			if let Some(album) = &mut model.album {
				//Load only X pictures in DOM 
				album.pictures.iter_mut()
					.filter(|p| !p.dom)
					.take(MAX_LOAD)
					.for_each(|p| p.dom = true);
				model.loaded += MAX_LOAD;
				//Load pictures
				album.pictures.clone()
					.into_iter()
					.filter(|p| p.dom)
					.for_each(|p| {
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
	let s_pic_list = style! {
		St::Display => "flex",
		St::FlexFlow => "row wrap",
		St::AlignItems => "center",
	};
	let s_pic = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
	};
	let s_pic_border = style! {
		St::Margin => rem(0.5),
		St::BorderRadius => rem(0.2),
	};
	let s_pic_img = style! {
		St::MaxWidth => rem(15),
	};
	let s_pic_empty = style! {
		St::Width => rem(5),
		St::Height => rem(5),
		St::Background => "rgba(0, 0, 0, 0.2)",
	};
	let s_caption = style! {
		St::TextAlign => "center",
	};
	let s_loader = style! {
		St::Position => "absolute",
		St::MarginLeft => rem(1),
		St::MarginTop => rem(1),
		St::Width => rem(3),
		St::Height => rem(3),
		St::Background => "rgba(0, 0, 0, 0.2)",
		St::BorderRadius => percent(50),
	};
	let s_loader_1 = style! {
		St::Transform => "scale(1)",
		St::Animation => "pulse 2s infinite linear",
	};
	let s_loader_2 = style! {
		St::Transform => "scale(0)",
		St::Animation => "pulse 2s 1s infinite linear",
	};
	let s_footer = style! {
		St::Display => "flex",
		St::JustifyContent => "center",
		St::Margin => rem(1),
	};
	let s_footer_btn = style! {
		St::Color => "#008891",
		St::FontSize => rem(2),
		St::TextShadow => "2px 1px 2px rgba(0, 0, 0, 0.3)",
		St::Animation => "movebtn 1s infinite linear",
	};
	nodes![
		match &model.album {
			Some(album) => div![
				s_album,
				h1![
					s_title,
					&album.name
				],
				div![
					s_pic_list,
					album.pictures.iter().filter(|p| p.dom).map(|pic| div![
						&s_pic,
						match &pic.data {
							Some(data_url) => img![
								&s_pic_border,
								&s_pic_img,
								attrs!{ At::Src => data_url }
							],
							_ => div![
								&s_pic_border,
								&s_pic_empty,
								div![&s_loader, &s_loader_1 ],
								div![&s_loader, &s_loader_2 ],
							]
						},
						span![
							&s_caption,
							&pic.caption
						],
					]),
				],
				IF!(album.pictures.len() > model.loaded => div![
					s_footer,
					i![
						s_footer_btn,
						C!("fa fa-chevron-down album__footer_btn"),
						ev(Ev::Click, |_| Msg::LoadPictures),
					],
				]),
			],
			None => empty![],
		},
	]
}