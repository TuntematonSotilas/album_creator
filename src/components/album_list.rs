use seed::{self, prelude::*, *};

use crate::conf::vars::API_URI;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

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
				let request = format!("{0}get-albums", API_URI);
				let response = fetch(request).await;
				


                //Msg::Received(user)
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