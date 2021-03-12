use seed::{self, prelude::*, *};

use crate::{
	utils::{
		style::*,
		request::{get_album, get_picture}, 
		pubvars::{MAX_LOAD, SWITCH_TIMEOUT},
	},
	models::album::Album,
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Option<Album>,
	loaded: usize,
	is_switched: bool,
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
	Switch,
	GoToEdit(String),
	AnimBckg(bool),
	Play,
	GoToPlay(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(id_url) => {
			model.album = None;
			model.loaded = 0;
			model.is_switched = false;
			orders.skip(); // No need to rerender
			let mut album_opt: Option<Album> = None;
			orders.perform_cmd(async {
				if let Some(id) = id_url {
					album_opt = get_album(id).await;
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
				album.pictures.iter()
					.filter(|p| p.dom && p.id.is_some())
					.for_each(|p| {
						orders.send_msg(Msg::GetPicture(p.id.clone().unwrap()));
					});
			}
		},
		Msg::GetPicture(id) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
				let id_c = id.clone(); 
				let data_opt = get_picture(id_c).await;
				Msg::PictureReceived(data_opt, id)
			});
		},
		Msg::PictureReceived(data, id) => {
			if let Some(album) = &mut model.album {
				album.pictures.iter_mut()
					.find(|p| p.id == Some(id.clone()))
					.map(|p| p.data = data);
			}
		},
		Msg::Switch => {
			orders.send_msg(Msg::AnimBckg(true));
			model.is_switched = true;
			if let Some(album) = &model.album {
				let frid = album.frid.clone();
				orders.perform_cmd(cmds::timeout(SWITCH_TIMEOUT, ||Msg::GoToEdit(frid)));	
			}
		},
		Msg::GoToEdit(_frid) => (),
		Msg::AnimBckg(_is_edit) => (),
		Msg::Play => {
			if let Some(album) = &model.album {
				let frid = album.frid.clone();
				orders.send_msg(Msg::GoToPlay(frid));
			}
		},
		Msg::GoToPlay(_frid) => (),
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_main = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
	};
	let s_header = style! {
		St::Display => "flex",
		St::Width => vw(90),
		St::Height => rem(5),
		St::AlignItems => "center",
	};
	let s_title = style! {
		St::TextAlign => "center",
		St::FontSize => rem(2),
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
		St::Width => vw(85),
	};
	let s_pic_list = style! {
		St::Display => "flex",
		St::FlexFlow => "row wrap",
		St::AlignItems => "center",
		St::FontSize => rem(0.8),
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
	
	let s_footer = style! {
		St::Display => "flex",
		St::JustifyContent => "center",
		St::Margin => rem(1),
	};
	
	nodes![
		match &model.album {
			Some(album) => div![
				s_main,
				div![
					s_header,
					a![
						C!("btn_icon btn_icon--blue"),
						s_btn_icon(Size::S, 1),
						ev(Ev::Click, |_| Msg::Play),
						i![
							C!("fa fa-play"),
						],
						attrs! { At::Href => String::new()},
					],
					div![
						s_title,
						&album.name
					],
					div![
						s_switch(),
						s_switch_anim(model.is_switched),
						span![
							s_switch_btn(), 
							s_switch_btn_anim(model.is_switched),
						],
						ev(Ev::Click, |_| Msg::Switch),
					],
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
								div![s_loader(), s_loader_1() ],
								div![s_loader(), s_loader_2() ],
							]
						},
						IF!(pic.caption.is_some() => 
							span![
								&s_caption,
								pic.caption.clone().unwrap()
							]
						),
					]),
				],
				IF!(album.pictures.len() > model.loaded => div![
					s_footer,
					div![
						C!("button"),
                        s_button(),
						ev(Ev::Click, |_| Msg::LoadPictures),
						"Load more"
					]
				]),
			],
			None => empty![],
		},
	]
}