use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	show: bool,
}

// ------------
//    Update
// ------------

pub enum Msg {
	SetIsAuth(bool),
	SetIsAuth2,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::SetIsAuth(_is_auth) => {
			orders.stream_with_handle(streams::interval(3000, || Msg::SetIsAuth2));
		},
		Msg::SetIsAuth2 => {
			log!("a");
			model.show = true;
		}
			
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_nav = style! {
        St::Display => "flex",
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
		St::Transition => "scale 5s ease;"
	};

	let s_anim = match model.show {
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
				class!("menu__item menu__item--blue"),
				s_item.clone(),
				s_anim,
                attrs! { At::Href => String::new() },
                i![
                    class!("fa fa-book-open"),
                ],
            ],
            a![
                class!("menu__item menu__item--green"),
                s_item.clone(),
                attrs! { At::Href => String::new() },
                i![
                    class!("fa fa-plus"),
                ],
            ],
        ]
    ]
}