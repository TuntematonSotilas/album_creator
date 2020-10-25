use seed::{self, prelude::*, *};

use crate::components::main::Page;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	anim: bool,
}

// ------------
//    Update
// ------------

pub enum Msg {
	SetIsAuth,
	Animate,
	ShowPage(Page),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::SetIsAuth => {
			orders.after_next_render(|_| Msg::Animate);
		},
		Msg::Animate => model.anim = true,
		Msg::ShowPage(_page) => (),
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
        St::BorderRadius => percent(100),
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
				s_item.clone(),
				s_anim.clone(),
                attrs! { At::Href => String::new() },
                i![
                    C!("fa fa-book-open"),
				],
				ev(Ev::Click, |_| Msg::ShowPage(Page::AlbumList)),
            ],
            a![
                C!("menu__item menu__item--green"),
				s_item.clone(),
				s_anim.clone(),
                attrs! { At::Href => String::new() },
                i![
                    C!("fa fa-plus"),
				],
				ev(Ev::Click, |_| Msg::ShowPage(Page::NewAlbum)),
            ],
        ]
    ]
}