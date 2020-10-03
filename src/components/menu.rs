use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    is_auth: bool,
}

// ------------
//    Update
// ------------

pub enum Msg {
    SetIsAuth(bool),
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetIsAuth(is_auth) => model.is_auth = is_auth,
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        match model.is_auth {
            true => span!["menu"], 
            false => empty![],
        }
    ]
}