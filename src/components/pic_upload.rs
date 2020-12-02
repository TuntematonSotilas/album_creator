use seed::{self, prelude::*, *};

use crate::utils::style::s_button;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

// ------------
//    Update
// ------------

pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
	let s_input_file = style! {
		St::Display => "none",
	};
	let s_btn = style! {
		St::MarginTop => rem(1),
	};
	nodes![
		label![
			s_btn,
			s_button(),
			C!("button"),
            "Upload",
            input![
                s_input_file,
                attrs! {
                    At::Type => "file",
                    At::Accept => "image/*",
                },
            ],
        ],
	]
}