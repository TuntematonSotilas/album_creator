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
	nodes![
		div![
			input![
				attrs! {
					At::Value => model.album.name,
					At::Placeholder => "Name",
				},
				input_ev(Ev::Blur, Msg::NameBlur),
			],
			span![
				match model.status {
					Status::New => "New",
					Status::Saving => "Saving",
					Status::Saved => "Saved",
					Status::Error => "Error",
				}
			]
		]
		
	]
}