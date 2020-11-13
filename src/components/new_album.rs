use seed::{self, prelude::*, *};

use friendly_id;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    id: String,
}


impl Model {
	pub fn new() -> Self {
		Model {
			id: friendly_id::create(),
		}
	}
}

// ------------
//    Update
// ------------

pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	nodes![
		span![&model.id],
	]
}