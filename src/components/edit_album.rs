use seed::{self, prelude::*, *};

use friendly_id;

use crate::utils::{vars::API_URI, request::get_auth};

// ------------
//     Model
// -----------

pub struct Model {
	album: Album,
	status: Status
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
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show => {
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

			let uri = format!("{0}new-album", API_URI);
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
		},
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let (status, color, bckg) = match model.status {
		Status::New => (
			"NEW".to_string(),
			"#0396ff".to_string(),
			"#b5d7ff".to_string()),
		Status::Saving => (
			"SAVING".to_string(),
			"#ffb103".to_string(),
			"#a49582".to_string()),
		Status::Saved => (
			"SAVED".to_string(), 
			"#0ad406".to_string(),
			"#c3ffdc".to_string()),
		Status::Error => (
			"ERROR".to_string(), 
			"#794242".to_string(),
			"#d98882".to_string()),
	};
	let s_column = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
	};
	let s_row = style! {
		St::Display => "flex",
		St::FlexDirection => "row",
		St::AlignItems => "center",
		St::Margin => rem(1),
	};
	let s_input = style! {
		St::Outline => "none",
		St::FontSize => vh(3),
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::Border => "none",
		St::Background => "none",
		St::TextAlign => "center",
		St::Width => percent(80),
	};
	let s_status = style! {
		St::Color => color,
		St::Background => bckg,
		St::BorderRadius => rem(0.25),
		St::BorderColor => color,
		St::BorderWidth => px(1),
		St::BorderStyle => "solid",
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::FontSize => rem(0.7),
		St::Padding => rem(0.4),
		St::LetterSpacing => rem(0.2),
	};
	nodes![
		div![
			s_column,
			div![
				s_row,
				input![
					s_input,
					attrs! {
						At::Value => model.album.name,
						At::Placeholder => "Name",
						At::MaxLength => 20,
					},
					input_ev(Ev::Blur, Msg::NameBlur),
				],
				div![
					s_status,
					status
				],
			],
			span!["photos"]
		],
	]
}