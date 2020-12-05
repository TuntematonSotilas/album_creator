use seed::{self, prelude::*, *};

use friendly_id;

use crate::{
	components::pic_upload,
	utils::{
		vars::API_URI, 
		request::get_auth,
	},
};

// ------------
//     Model
// -----------

pub struct Model {
	album: Album,
	status: Status,
	pic_upload: pic_upload::Model,
}

#[derive(serde::Serialize, Default)]
struct Album {
	frid: String,
	name: String,
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
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show => {
			model.status = Status::New;
			model.album.frid = friendly_id::create();
			model.album.name = "New Album".to_string();
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
				.json(&model.album);
			
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
				pic_upload::Msg::SetUploadResult(ref pic_opt) => {
					if let Some(picture) = pic_opt {
						log!(picture.id);
					}
				},
				_ => ()
			}
			pic_upload::update(msg, &mut model.pic_upload, &mut orders.proxy(Msg::PicUpload));
		}
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let (status, color) = match model.status {
		Status::New => (
			"NEW".to_string(),
			"#0396ff".to_string()),
		Status::Saving => (
			"SAVING".to_string(),
			"#788d95".to_string()),
		Status::Saved => (
			"SAVED".to_string(), 
			"#0ad406".to_string()),
		Status::Error => (
			"ERROR".to_string(), 
			"#794242".to_string()),
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
			pic_upload::view(&model.pic_upload).map_msg(Msg::PicUpload),
		],
	]
}