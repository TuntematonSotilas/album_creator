use seed::{self, prelude::*, *};

use friendly_id;

use crate::utils::{vars::API_URI, request::get_auth};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Album,
}

#[derive(serde::Serialize, Default)]
struct Album {
	frid: String,
	name: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show,
	NameChanged(String),
	Post,
	Toast(bool),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show => {
			model.album.frid = friendly_id::create();
			model.album.name = "New Album".to_string();
		},
		Msg::NameChanged(name) => {
			model.album.name = name;
			orders.send_msg(Msg::Post);
		},
		Msg::Post => {
			orders.skip(); // No need to rerender
			
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
				Msg::Toast(is_success)
			});
		},
		Msg::Toast(is_success) => {
			log!("Toast", is_success);
		},
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	nodes![
		form![
			input![
				attrs! {
					At::Value => model.album.name,
					At::Placeholder => "Name",
				},
				input_ev(Ev::Input, Msg::NameChanged),
			],
		],		
	]
}