#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::{
    login,
    menu,
    toast,
    album_list,
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
        menu: menu::Model::default(),
        toast: toast::Model::default(),
        album_list: album_list::Model::default(),
        //base_url: url.to_base_url(),
        page: Page::init(url),
        is_auth: true,
    }
}

// ------------
//     Model
// ------------

struct Model {
    login: login::Model,
    menu: menu::Model,
    toast: toast::Model,
    album_list: album_list::Model,
    //base_url: Url,
    page: Page,
    is_auth: bool,
}

enum Page {
    Login,
    Menu,
    AlbumList,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Login,
            Some("menu") => Self::Menu,
            Some("albums") => Self::AlbumList,
            Some(_) => Self::Login,
        }
    }
}

// ------------
//    Update
// ------------

enum Msg {
    Login(login::Msg),
    Menu(menu::Msg),
    Toast(toast::Msg),
    AlbumList(album_list::Msg),
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
                login::Msg::SetIsAuth => model.page = Page::Menu,
                login::Msg::ShowToast(ref toast) => {
                    orders.send_msg(Msg::ShowToast(toast.clone()));
                },
                _ => (),
            };
            login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
        },
        Msg::Toast(msg) => {
            toast::update(msg, &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
        Msg::ShowToast(toast) => {
            toast::update(toast::Msg::Show(toast), &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
        Msg::AlbumList(msg) => {
			album_list::update(msg, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
        },
        Msg::Menu(msg) => {
			menu::update(msg, &mut model.menu, &mut orders.proxy(Msg::Menu));
		},
    }
}

// ------------
//     View
// ------------

fn view(model: &Model) -> Node<Msg> {
    let style = style! { 
        St::Height => vh(100),
        St::FontFamily => "'Open Sans', sans-serif",
    };
    div![style,
        toast::view(&model.toast).map_msg(Msg::Toast),
        /*li![a![
            attrs! { At::Href => model.base_url },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => model.base_url.clone().add_path_part("albums") },
            "Albums",
        ]],*/
        match &model.page {
            Page::Login => login::view(&model.login).map_msg(Msg::Login),
            Page::Menu => menu::view(&model.menu).map_msg(Msg::Menu),
            Page::AlbumList => album_list::view(&model.album_list).map_msg(Msg::AlbumList),
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
