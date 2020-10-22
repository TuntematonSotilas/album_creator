use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

// ------------
//    Update
// ------------

pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
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
        St::BoxShadow => "3px 3px 0 0 rgba(0, 0, 0, 0.14)"
    };
    nodes![
        nav![
            s_nav,
            a![
                class!("menu__item menu__item--blue"),
                s_item.clone(),
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