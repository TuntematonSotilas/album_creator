#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::{
    login, 
    menu,
    toast,
};
use crate::models::toast::Toast;

mod components;
mod conf;
mod models;

// ------------
//     Init
// ------------

// `init` describes what should happen when your app started.
fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        login: login::Model::default(),
        menu: menu::Model::default(),
        toast: toast::Model::default(),
    }
}

// ------------
//     Model
// ------------

#[derive(Default)]
struct Model {
    login: login::Model,
    menu: menu::Model,
    toast: toast::Model,
}

// ------------
//    Update
// ------------

enum Msg {
    Login(login::Msg),
    Menu(menu::Msg),
    Toast(toast::Msg),
    SetIsAuth(bool),
    ShowToast(Toast),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login(msg) => {
            match msg {
                login::Msg::SetIsAuth(is_auth) => orders.send_msg(Msg::SetIsAuth(is_auth)),
                login::Msg::ShowToast(ref toast) => orders.send_msg(Msg::ShowToast(toast.clone())),
                _ => orders.skip(),
            };
            login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
        },
        Msg::Menu(msg) => {
            menu::update(msg, &mut model.menu, &mut orders.proxy(Msg::Menu));
        },
        Msg::Toast(msg) => {
            toast::update(msg, &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
        Msg::SetIsAuth(is_auth) => {
            menu::update(menu::Msg::SetIsAuth(is_auth), &mut model.menu, &mut orders.proxy(Msg::Menu));
        },
        Msg::ShowToast(toast) => {
            toast::update(toast::Msg::Show(toast), &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
    }
}

// ------------
//     View
// ------------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let style = style! { 
        St::Height => vh(100),
        St::FontFamily => "'Open Sans', sans-serif",
    };
    div![style,
        toast::view(&model.toast).map_msg(Msg::Toast),
        login::view(&model.login).map_msg(Msg::Login),
        menu::view(&model.menu).map_msg(Msg::Menu),
    ]
}

// ------------
//     Start
// ------------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
