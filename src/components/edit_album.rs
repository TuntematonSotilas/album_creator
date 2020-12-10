use seed::{self, prelude::*, *};

use friendly_id;

use crate::{
	components::pic_upload,
	utils::{
		vars::API_URI, 
		request::get_auth,
		serializer::ser_edit_album,
	},
	models::picture,
};

// ------------
//     Model
// -----------

pub struct Model {
	album: Album,
	status: Status,
	pic_upload: pic_upload::Model,
}

#[derive(Default, Clone)]
pub struct Album {
	pub frid: String,
	pub name: String,
	pub pictures : Vec<picture::Picture>,
}

enum Status {
	New,
	Saving,
	Saved,
	Error,
}

impl Model {
	pub fn new() -> Self {
		Model {
			album: Album::default(),
			status: Status::New,
			pic_upload: pic_upload::Model::default(),
		}
	}
}
// ------------
//    Update
// ------------

pub enum Msg {
	Show,
	NameBlur(String),
	Post,
	SetStatus(bool),
	PicUpload(pic_upload::Msg),
	CaptionBlur(Option<String>, String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show => {
			model.status = Status::New;
			model.album.frid = friendly_id::create();
			model.album.name = "New Album".into();
		},
		Msg::NameBlur(name) => {
			model.album.name = name;
			orders.send_msg(Msg::Post);
		}
		Msg::Post => {
			orders.skip(); // No need to rerender
			
			model.status = Status::Saving;

			let uri = format!("{0}edit-album", API_URI);
			let request = Request::new(uri)
				.method(Method::Post)
				.header(Header::authorization(get_auth()))
				.json(&ser_edit_album(model.album.clone()));
			
			orders.perform_cmd(async {
				let mut is_success = false;
				if let Ok(json) = request {
					let result = fetch(json).await;
					if let Ok(response) = result {
						if let Ok(_) = response.check_status() {
							is_success = true;
						}
					}
				}
				Msg::SetStatus(is_success)
			});
		},
		Msg::SetStatus(is_success) => {
			model.status = match is_success {
				true => Status::Saved,
				false => Status::Error,
			};
			if is_success {
				pic_upload::update(pic_upload::Msg::Show(model.album.frid.clone(), 0), &mut model.pic_upload, &mut orders.proxy(Msg::PicUpload));
			}
		},
		Msg::PicUpload(msg) => {
			match msg {
				pic_upload::Msg::BeginUpload(ref pic_opt) => {
					if let Some(picture) = pic_opt.clone() {
						model.album.pictures.push(picture);
					}
				},
				pic_upload::Msg::EndUpload(ref id_opt) => {
					model.album.pictures.iter_mut()
						.filter(|p| p.id == None)
						.for_each(|p| p.id = id_opt.clone());
				},
				_ => ()
			}
			pic_upload::update(msg, &mut model.pic_upload, &mut orders.proxy(Msg::PicUpload));
		},
		Msg::CaptionBlur(pic_id, caption) => {
			model.album.pictures.iter_mut()
				.filter(|p| p.id == pic_id)
				.for_each(|p| p.caption = Some(caption.clone()));
		}
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let (status, color): (String, String) = match model.status {
		Status::New => (
			"NEW".into(),
			"#0396ff".into()),
		Status::Saving => (
			"SAVING".into(),
			"#788d95".into()),
		Status::Saved => (
			"SAVED".into(), 
			"#0ad406".into()),
		Status::Error => (
			"ERROR".into(), 
			"#794242".into()),
	};
	let s_status = style! {
		St::Color => color,
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::FontSize => rem(0.7),
		St::LetterSpacing => rem(0.2),
		St::TransitionProperty => "color",
		St::TransitionDuration => "200ms",
		St::AlignSelf => "start",
	};
	let s_column = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
		St::MarginTop => rem(0.5),
	};
	let s_info_ctn = style! {
		St::Width => vw(90),
	};
	let s_infos = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
		St::Border => "1px solid #23888e",
		St::BorderRadius => rem(0.3),
		St::BoxShadow => "2px 2px 2px rgba(35, 136, 142, 0.5)",
	};
	let s_name = style! {
		St::Outline => "none",
		St::FontSize => rem(2),
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::Border => "none",
		St::Background => "none",
		St::TextAlign => "center",
	};
	let s_list = style! {
		St::ListStyle => "none",
		St::Padding => 0,
		St::Display => "flex",
		St::FlexDirection => "column",
		St::Width => vw(90),
	};
	let s_pic = style! {
		St::MaxWidth => rem(6),
	};
	nodes![
		div![
			s_column,
			div![
				s_info_ctn,
				span![
					s_status,
					status
				],
				div![
					s_infos,
					input![
						s_name,
						attrs! {
							At::Value => model.album.name,
							At::Placeholder => "Name",
							At::MaxLength => 20,
						},
						input_ev(Ev::Blur, Msg::NameBlur),
					],
				],
			],
			ul![
				s_list,
				model.album.pictures.iter().map(|p| {
					let mut caption = String::new();
					if let Some(cap) = p.caption.clone() {
						caption = cap;
					};
					let id = p.id.clone();
					li![
						img![
							&s_pic,
							attrs!{ At::Src => p.data }
						],
						input![
							attrs! {
								At::Value => caption,
								At::Placeholder => "Caption",
								At::Disabled => p.id.is_none().as_at_value(),
							},
							input_ev(Ev::Blur, |value| Msg::CaptionBlur(id, value)),
						],
					]
				}),
			],
			pic_upload::view(&model.pic_upload).map_msg(Msg::PicUpload),
		],
	]
}