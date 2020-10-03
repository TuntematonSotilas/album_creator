use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
}

// ------------
//    Update
// ------------

pub enum Msg {
    SetIsAuth(bool),
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetIsAuth(is_auth) => log!("menu", is_auth),
    }
}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
    nodes![
        span!["menu"],
    ]
}