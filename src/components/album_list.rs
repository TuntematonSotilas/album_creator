use seed::{self, prelude::*, *};

use crate::utils::{
	vars::API_URI,
	request::get_auth
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

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
	Fetch,
}

pub fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::Fetch => {
			orders.skip(); // No need to rerender
            orders.perform_cmd(async {
				let uri = format!("{0}get-albums", API_URI);
				let request = Request::new(uri)
                	.method(Method::Get)
					.header(Header::authorization(get_auth()));
				let response_res = fetch(request).await;
				if let Ok(response) = response_res {
					if let Ok(resp_ok) = response.check_status() {
						let albums_res = resp_ok.json::<Album>().await;
						if let Ok(albums) = albums_res {

						}
					}
				}	
            });
		},
	}
}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
	nodes![
		h1!["ablums"],
	]
}