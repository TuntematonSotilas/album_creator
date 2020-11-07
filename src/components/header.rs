use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	anim: bool,
	base_url: Url,
}


impl Model {
	pub fn new(url: Url) -> Self {
		Model {
			anim: false,
			base_url: url.to_base_url(),
		}
	}
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show,
	Animate,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::Show => {
			log!("header show");
			model.anim = false;
			orders.after_next_render(|_| Msg::Animate);
		},
		Msg::Animate => model.anim = true,
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_header = style! {
		St::TextAlign => "center",
		St::Background => "radial-gradient(circle at top left, rgba(130, 130, 130, 0.5) 0%, rgba(0,0,0,0) 100%), #4474ad",
		St::BoxShadow => "0 0px 2px rgba(0, 0, 0, 0.5)",
		St::FontSize => rem(1),
		St::Color => "white",
		St::LetterSpacing => rem(0.1),
		St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
		St::Transition => "padding 200ms ease-out",
		St::TransitionTimingFunction => "cubic-bezier(0.2, 0.8, 0.3, 1.2)",
	};
	let s_anim = match model.anim {
		true => style! { 
			St::Padding => rem(0.5),
		},
		false => style! { 
			St::Padding => rem(0),
		},
	};
	nodes![
		header![
			s_header,
			s_anim,
			a![
				C!["header__link"],
				attrs! { At::Href => model.base_url.clone().add_path_part("menu") },
				"Album Creator",
			]
		],
	]
}