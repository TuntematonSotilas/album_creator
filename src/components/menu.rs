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
    let s_item = style! {
        St::BorderRadius => percent(100),
        St::Width => rem(20),
        St::Height => rem(20),
    };
    nodes![
        nav![
            a![
                i![
                    s_item.clone(),
                    class!("fa fa-book-open"),
                ],
                i![
                    s_item.clone(),
                    class!("fa fa-plus"),
                ],
            ],
        ]
    ]
}