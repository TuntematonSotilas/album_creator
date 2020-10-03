#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::login;
use crate::utils::conf_util;

mod components;
mod utils;

// ------------
//     Init
// ------------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        login: login::Model::new(conf_util::parse_conf()),
    }
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
