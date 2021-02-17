use seed::{self, prelude::*, *};

use crate::utils::style::{Size, s_btn_icon};

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
				C!("btn_icon btn_icon--blue"),
				&s_btn_icon(Size::X, 1),
				&s_anim,
				attrs! { At::Href => model.base_url.clone().add_path_part("albums") },
                i![
                    C!("fa fa-book-open"),
				],
            ],
            a![
                C!("btn_icon btn_icon--green"),
				&s_btn_icon(Size::X, 1),
				&s_anim,
                attrs! { At::Href => model.base_url.clone().add_path_part("edit") },
                i![
                    C!("fa fa-plus"),
				],
            ],
        ]
    ]
}