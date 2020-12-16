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
		St::ZIndex => 1,
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
	let s_message = style! {
		St::FlexGrow => 1,
		St::MarginTop => percent(18),
	};
	let s_buttons = style! {
		St::Display => "flex",
		St::Width => percent(100),
	};
	let s_button = style! {
		St::Background => "none",
		St::Border => "none",
		St::Width => percent(100),
		St::Height => rem(2.5),
		St::Cursor => "pointer",
	};
	let s_yes = style! {
		St::Background => "#fc7169",
		St::BorderRadius => "0 0 0 0.2rem",
	};
	let s_no = style! {
		St::Background => "#b6bece",
		St::BorderRadius => "0 0 0.2rem 0",
	};
    nodes![
        div![
			s_confirm,
            s_anim,
            div![
                s_modal,
				div![
					s_message,
					&model.message
				],
				div![
					s_buttons,
					button![
						C!("confirm__yes"),
						&s_button,
						s_yes,
						"YES",
						ev(Ev::Click, |_| Msg::Ok),
					],
					button![
						C!("confirm__no"),
						&s_button,
						s_no,
						"NO",
						ev(Ev::Click, |_| Msg::Close),
					],
				]
            ],
        ]
    ]
}