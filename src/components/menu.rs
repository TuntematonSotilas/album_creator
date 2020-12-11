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
	pub fn new(url: &Url) -> Self {
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
    let s_nav = style! {
		St::Display => "flex",
		St::JustifyContent => "center",
		St::MarginTop => vh(10),
    };
    let s_item = style! {
        St::Width => rem(5),
        St::Height => rem(5),
        St::Margin => rem(1),
        St::BorderRadius => rem(0.5),
        St::FontSize => rem(1.5),
        St::TextAlign => "center",
        St::LineHeight => rem(5),
		St::BoxShadow => "3px 3px 0 0 rgba(0, 0, 0, 0.14)",
		St::Transition => "scale 200ms ease-out",
		St::TransitionTimingFunction => "cubic-bezier(0.2, 0.8, 0.3, 1.2)",
	};
	let s_anim = match model.anim {
		true => style! { 
			St::Scale => 1
		},
		false => style! { 
			St::Scale => 0
		},
	};
    nodes![
        nav![
            s_nav,
            a![
				C!("menu__item menu__item--blue"),
				&s_item,
				&s_anim,
				attrs! { At::Href => model.base_url.clone().add_path_part("albums") },
                i![
                    C!("fa fa-book-open"),
				],
            ],
            a![
                C!("menu__item menu__item--green"),
				&s_item,
				&s_anim,
                attrs! { At::Href => model.base_url.clone().add_path_part("new") },
                i![
                    C!("fa fa-plus"),
				],
            ],
        ]
    ]
}