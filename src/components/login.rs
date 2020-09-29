use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    result: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
    Login,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login => {
            model.result = "Ok".to_string();
        }
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div!["login"],
        span![model.result.clone()]
        button![ev(Ev::Click, |_| Msg::Login), "Login"],
    ]
}