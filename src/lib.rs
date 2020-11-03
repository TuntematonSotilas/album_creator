#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::{
    login, 
    main,
    toast,
};
use crate::models::toast::Toast;

mod components;
mod utils;
mod models;

// ------------
//     Init
// ------------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        login: login::Model::default(),
        main: main::Model::new(),
        toast: toast::Model::default(),
        base_url: url.to_base_url(),
        page: Page::init(url),
    }
}

// ------------
//     Model
// ------------

struct Model {
    login: login::Model,
    main: main::Model,
    toast: toast::Model,
    base_url: Url,
    page: Page,
}

enum Page {
    Home,
    AlbumList,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some("albums") => Self::AlbumList,
            Some(_) => Self::Home,
        }
    }
}

// ------------
//    Update
// ------------

enum Msg {
    Login(login::Msg),
    Main(main::Msg),
    Toast(toast::Msg),
    SetIsAuth(bool),
    ShowToast(Toast),
    UrlChanged(subs::UrlChanged),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::Login(msg) => {
            match msg {
                login::Msg::SetIsAuth(is_auth) => orders.send_msg(Msg::SetIsAuth(is_auth)),
                login::Msg::ShowToast(ref toast) => orders.send_msg(Msg::ShowToast(toast.clone())),
                _ => orders.skip(),
            };
            login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
        },
        Msg::Main(msg) => {
            main::update(msg, &mut model.main, &mut orders.proxy(Msg::Main));
        },
        Msg::Toast(msg) => {
            toast::update(msg, &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
        Msg::SetIsAuth(is_auth) => {
            main::update(main::Msg::SetIsAuth(is_auth), &mut model.main, &mut orders.proxy(Msg::Main));
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
        /*toast::view(&model.toast).map_msg(Msg::Toast),
        login::view(&model.login).map_msg(Msg::Login),
        main::view(&model.main).map_msg(Msg::Main),*/

        li![a![
            attrs! { At::Href => model.base_url },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => model.base_url.clone().add_path_part("albums") },
            "Albums",
        ]],
        match &model.page {
            Page::Home => div!["home"],
            Page::AlbumList => div!["albums"]
        }
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
