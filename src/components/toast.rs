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
    let s_toast = style!{
        St::Position => "fixed",
        St::Left => percent(50),
        St::Width => rem(20),
        St::MarginLeft => rem(-10),
        St::MarginTop => rem(0.5),
        St::Padding => ".75rem 1.25rem",
        St::Color => "#ff0303",
        St::Background => "rgba(220, 17, 1, 0.3)",
        St::Border => "1px solid rgba(241, 6, 6, 0.81)",
        St::BorderRadius => "0.25rem",
        St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
    };
    let s_title= style! {
        St::MarginLeft => rem(0.7);
        St::MarginRight => rem(0.7);
    };
    nodes![
        if model.toast.is_visible && 
            model.toast.title.is_some() && 
            model.toast.content.is_some() {
                div![
                    class!("toast"),
                    s_toast,
                    i![
                        class!("far fa-times-circle"),
                    ],
                    strong![
                        s_title,
                        model.toast.clone().title.unwrap()
                    ],
                    model.toast.clone().content.unwrap()
                ]
        } else {
            empty![]
        },
    ]
}