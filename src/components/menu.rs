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
    nodes![
        nav![
            a![
                i![
                    class!("fa fa-heart"),
                ],
            ],
        ]
    ]
}