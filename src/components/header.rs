use seed::{self, prelude::*, *};

use crate::Page;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

// ------------
//    Update
// ------------

pub enum Msg {
	ShowPage(Page),
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::ShowPage(_page) => (),
	}
}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
	let s_header = style! {
		St::Padding => rem(0.5),
		St::TextAlign => "center",
		St::Background => "radial-gradient(circle at top left, rgba(130, 130, 130, 0.5) 0%, rgba(0,0,0,0) 100%), #4474ad",
		St::BoxShadow => "0 0px 2px rgba(0, 0, 0, 0.5)",
		St::FontSize => rem(1),
		St::Color => "white",
		St::LetterSpacing => rem(0.1),
        St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
	};
	nodes![
		header![
			s_header,
			a![
				C!["header__link"],
				attrs! { At::Href => String::new() },
				"Album Creator",
				ev(Ev::Click, |_| Msg::ShowPage(Page::Menu)),
			]
		],
	]
}