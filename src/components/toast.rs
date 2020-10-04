use seed::{self, prelude::*, *};

use crate::models::toast::Toast;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    toast: Toast,
}

// ------------
//    Update
// ------------

pub enum Msg {
    Show(Toast),
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Show(toast) => model.toast = toast,
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let style = style!{
        St::Position => "fixed",
    };
    nodes![
        if let Some(message) = &model.toast.message {
            div![
                style,
                message
            ]
        } else {
            empty![]
        },
    ]
}