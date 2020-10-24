use seed::{self, prelude::*, *};

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
	let s_header = style! {
		St::Padding => rem(0.5),
		St::TextAlign => "center",
		St::Background => "#4474ad",
		St::BoxShadow => "0 0px 2px rgba(0, 0, 0, 0.5)",
		St::FontSize => rem(1),
		St::Color => "white",
		St::LetterSpacing => rem(0.1),
        St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
	};
	nodes![
		header![
			s_header,
			"Album Creator"
		],
	]
}