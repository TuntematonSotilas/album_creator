use seed::{self, prelude::*, *};

use crate::utils::style::s_button;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    is_visible: bool,
    message: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show(String),
	Close,
	Ok,
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Show(message) => {
            model.message = message;
            model.is_visible = true;
		},
		Msg::Close => {
			model.is_visible = false;
		},
		Msg::Ok => {
			model.is_visible = false;
		},
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_confirm = style!{
        St::Position => "fixed",
        St::Background => "rgba(0, 0, 0, 0.5)",
        St::Transition => "opacity 600ms ease-out",
        St::Width => percent(100),
        St::Height => percent(100),
        St::AlignItems => "center",
        St::JustifyContent => "center",
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
    };
    let s_modal = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::Background => "radial-gradient(circle at top right, #c3e6e8 -30%, #008790 100%)",
        St::BorderRadius => rem(0.2),
        St::Width => rem(15),
        St::Height => rem(8),
    };
	let s_anim = match model.is_visible {
		true => style! {
            St::Opacity => 1
            St::Display => "flex",
		},
		false => style! {
            St::Opacity => 0,
            St::Display => "none",
		},
	};
	let s_buttons = style! {
		St::MarginTop => rem(1),
		St::Display => "flex",
	};
	let s_separator = style! {
		St::Width => rem(2);
	};
    nodes![
        div![
			s_confirm,
            s_anim,
            div![
                s_modal,
				span![
					&model.message],
				div![
					s_buttons,
					button![
						"No",
						C!("button"),
						s_button(),
						ev(Ev::Click, |_| Msg::Close),
					],
					span![s_separator],
					button![
						"Yes",
						C!("button"),
						s_button(),
						ev(Ev::Click, |_| Msg::Ok),
					],
				]
            ],
        ]
    ]
}