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
    nodes![
        div![
			s_confirm,
            s_anim,
            div![
                s_modal,
                span![&model.message],
            ],
        ]
    ]
}