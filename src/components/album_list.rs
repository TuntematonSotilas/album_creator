use seed::{self, prelude::*, *};

use crate::utils::{
	vars::API_URI,
	request::get_auth
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	albums: Option<Vec<Album>>,
}

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
	Received(Vec<Album>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
						let albums_res = resp_ok.json::<Vec<Album>>().await;
						if let Ok(albums) = albums_res {
							log!("ok");
							Msg::Received(albums)
						}
					}
				}	
            });
		},
		Msg::Received(albums) => {
			log!("Received");
			model.albums = Some(albums);
        }
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	nodes![
		h1!["ablums"],
		div![
			match &model.albums {
				Some(albums) => div![
					albums.iter().map(|album| div![
						attrs! { At::Id	=> album.id.oid },
						span![&album.name]
					])
				],
				None => div![span!["no albums"]],
			}
			
		],
	]
}