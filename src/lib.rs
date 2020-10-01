// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::login;

mod components;
mod config;

// ------------
//     Init
// ------------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------------
//     Model
// ------------

#[derive(Default)]
struct Model {
    login: login::Model,
}

// ------------
//    Update
// ------------

enum Msg {
    Login(login::Msg),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login(msg) => {
            login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
        }
    }
}

// ------------
//     View
// ------------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        login::view(&model.login).map_msg(Msg::Login),
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
