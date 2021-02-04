use seed::{self, prelude::*, *};

use friendly_id;

use crate::{
	components::pic_upload,
	utils::{
		vars::API_URI, 
		request::get_auth,
		serializer::{ser_edit_album, ser_edit_picture},
		style::s_button, 
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
	NameChange(String),
	Post,
	SetStatus(bool),
	PicUpload(pic_upload::Msg),
	CaptionChange(Option<String>, String),
	EditPicture(picture::Picture),
	Save,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::NameChange(name) => model.album.name = name,
		Msg::Show => {
			model.status = Status::New;
			model.album.frid = friendly_id::create();
			model.album.name = String::new();
			model.album.pictures = Vec::new();
		},
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
		Msg::CaptionChange(pic_id, caption) => {
			model.album.pictures.iter_mut()
				.filter(|p| p.id == pic_id)
				.for_each(|p| p.caption = Some(caption.clone()));
		},
		Msg::EditPicture(picture) => {
			orders.skip(); // No need to rerender

			model.status = Status::Saving;

			let uri = format!("{0}edit-picture", API_URI);
			let request = Request::new(uri)
				.method(Method::Post)
				.header(Header::authorization(get_auth()))
				.json(&ser_edit_picture(picture.clone()));
			
			orders.perform_cmd(async {
				let mut is_ok = false;
				if let Ok(json) = request {
					let result = fetch(json).await;
					if let Ok(response) = result {
						if response.check_status().is_ok() {
							is_ok = true;
						}
					}
				}
				Msg::SetStatus(is_ok)
			});
		},
		Msg::Save => {
			orders.send_msg(Msg::Post);
			model.album.pictures.iter()
				.for_each(|p| {
					orders.send_msg(Msg::EditPicture(p.clone()));
				});
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
	let s_column = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
		St::MarginTop => rem(0.5),
	};
	let s_status = style! {
		St::Color => color,
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::FontSize => rem(0.7),
		St::LetterSpacing => rem(0.2),
		St::TransitionProperty => "color",
		St::TransitionDuration => "200ms",
		St::AlignSelf => "center",
	};
	let s_header = style! {
		St::Display => "flex",
		St::JustifyContent => "space-between",
		St::Width => vw(90),
		St::Margin => rem(1),
	};	
	let s_panel = style! {
		St::AlignItems => "center",
		St::Display => "flex",
		St::Background => "radial-gradient(circle at bottom left, rgba(28, 28, 36, 0.5) 40%, transparent 120%)",
		St::BoxShadow => "inset 0.3rem 0rem 0.6rem #ffffff5c",
		St::BorderRadius => rem(0.3),
		St::Padding => rem(0.5),
		St::Width => vw(90),
	};
	let s_panel_name = style! {
		St::FlexDirection => "column",
	};
	let s_label = style! {
		St::Position => "relative",
		St::Top => rem(-1.4),
		St::Color => "#bdbdbd",
		St::Transition => "0.2s ease all",
		St::PointerEvents => "none",
	};
	let s_input = style! {
		St::Display => "block",
		St::MarginTop => rem(1),
		St::Outline => "none",
		St::Background => "none",
		St::Color => "#bdbdbd",
		St::LetterSpacing => rem(0.1),
		St::BorderBottom => "solid 1px #bdbdbd",
		St::BorderTop => "none",
		St::BorderLeft => "none",
		St::BorderRight => "none",
	};
	let s_name_input = style! {
		St::FontSize => rem(1.5),
	};
	let s_list = style! {
		St::ListStyle => "none",
		St::Padding => 0,
		St::Display => "flex",
		St::FlexDirection => "column",
	};
	let s_pic = style! {
		St::MaxWidth => rem(6),
		St::BorderRadius => rem(0.2),
	};
	let s_caption = style! {
		St::MarginLeft => rem(2),
	};
	nodes![
		div![
			s_column,
			div![
				s_header,
				span![
					s_status,
					status
				],
				button![
					s_button(),
					"Save",
					ev(Ev::Click, |_| Msg::Save),
				],
			],
			div![
				&s_panel,
				&s_panel_name,
				div![
					input![
						C!("edit_album__input"),
						&s_input,
						s_name_input,
						attrs! {
							At::Required => true,
							At::Value => model.album.name,
							At::MaxLength => 20,
						},
						input_ev(Ev::Input, Msg::NameChange),
					],
					label![
						&s_label,
						"Name",
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
						&s_panel,
						img![
							&s_pic,
							attrs!{ At::Src => p.data }
						],
						div![
							&s_caption,
							input![
								C!("edit_album__input"),
								&s_input,
								attrs! {
									At::Value => caption,
									At::Required => true,
									At::Disabled => p.id.is_none().as_at_value(),
								},
								input_ev(Ev::Input, |value| Msg::CaptionChange(id, value)),
							],
							label![
								&s_label,
								"Caption",
							],
						],
					]
				}),
			],
			pic_upload::view(&model.pic_upload).map_msg(Msg::PicUpload),
		],
	]
}