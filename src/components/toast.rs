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
        St::Left => percent(50),
        St::Width => rem(10),
        St::MarginLeft => rem(-6),
        St::Padding => ".75rem 1.25rem",
        St::Color => "#ff0303",
        St::Background => "rgba(220, 17, 1, 0.3)",
        St::Border => "1px solid rgba(241, 6, 6, 0.81)",
        St::BorderRadius => "0.25rem",
        St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
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