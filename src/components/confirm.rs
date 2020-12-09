use seed::{self, prelude::*, *};

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
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Show(message) => {
            model.message = message;
            model.is_visible = true;
        },
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_confirm = style!{
        St::Position => "fixed",
        St::Background => "rgba(0, 0, 0, 0.3)",
        St::Transition => "opacity 200ms ease-out",
    };
	let s_anim = match model.is_visible {
		true => style! {
			St::Opacity => 1
		},
		false => style! {
			St::Opacity => 0
		},
	};
    nodes![
        div![
			s_confirm,
			s_anim,
            span![&model.message],
        ]
    ]
}