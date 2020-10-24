use seed::{self, prelude::*, *};

use crate::components::menu;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    is_auth: bool,
    menu: menu::Model,
}

// ------------
//    Update
// ------------

pub enum Msg {
    SetIsAuth(bool),
    Menu(menu::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetIsAuth(is_auth) => {
			model.is_auth = is_auth;
			menu::update(menu::Msg::SetIsAuth, &mut model.menu, &mut orders.proxy(Msg::Menu));
		},
        Msg::Menu(msg) => {
            menu::update(msg, &mut model.menu, &mut orders.proxy(Msg::Menu));
        },
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_main = style![
        St::Background => "radial-gradient(circle at top left, #8bd2d6 -20%, #9bbade 100%)",
        St::Display => "flex",
        St::FlexDirection => "column",
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::Height => percent(100),
    ];
    nodes![
        match model.is_auth {
            true => div![
                s_main,
                menu::view(&model.menu).map_msg(Msg::Menu),
            ],
            false => empty![],
        }
    ]
}