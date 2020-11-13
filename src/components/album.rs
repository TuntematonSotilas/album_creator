use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}


// ------------
//    Update
// ------------

pub enum Msg {
	Show(Option<String>),
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(id_url) => {
			log!(id_url);
		},
	}
}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
	nodes![
		span!["album"],
	]
}