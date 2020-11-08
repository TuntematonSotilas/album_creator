#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::{
    login,
    toast,
	menu,
    header,
	album_list,
};
use crate::models::toast::Toast;

mod components;
mod utils;
mod models;

// ------------
//     Init
// ------------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
	orders.subscribe(Msg::UrlChanged)
		.notify(subs::UrlChanged(url.clone()));
    Model {
        login: login::Model::default(),
        toast: toast::Model::default(),
        menu: menu::Model::new(url.clone()),
		header: header::Model::new(url.clone()),
		album_list: album_list::Model::default(),
        base_url: url.to_base_url(),
        page: Page::init(url),
        is_auth: false,
    }
}

// ------------
//     Model
// ------------

struct Model {
    login: login::Model,
    toast: toast::Model,
    menu: menu::Model,
	header: header::Model,
	album_list: album_list::Model,
    base_url: Url,
    page: Page,
    is_auth: bool,
}

#[derive(Copy, Clone)]
pub enum Page {
    Login,
    Menu,
	AlbumList,
	NewAlbum,
}

impl Page {
    fn init(mut url: Url) -> Self {
		match url.next_path_part() {
            None => Self::Login,
            Some("menu") => Self::Menu,
			Some("albums") => Self::AlbumList,
			Some("new") => Self::NewAlbum,
            Some(_) => Self::Login,
        }
    }
}

// ------------
//    Update
// ------------

enum Msg {
    Login(login::Msg),
    Toast(toast::Msg),
	Header(header::Msg),
	Menu(menu::Msg),
    AlbumList(album_list::Msg),
    ShowToast(Toast),
	UrlChanged(subs::UrlChanged),
	SetUrl,
	LoadPage,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
			if !model.is_auth {
				model.page = Page::Login;
				orders.send_msg(Msg::SetUrl);
			} else {
				model.page = Page::init(url);
			}
			orders.send_msg(Msg::LoadPage);
		},
		Msg::SetUrl => {
			let path_part = match model.page {
				Page::Menu => "menu",
				Page::AlbumList => "ablums",
				Page::NewAlbum => "new",
				_ => "login",
			};
			//
			model.base_url.clone().add_path_part(path_part)
				.go_and_push();
		},
		Msg::LoadPage => {
			match model.page {
				Page::Menu => menu::update(menu::Msg::Show, &mut model.menu, &mut orders.proxy(Msg::Menu)),
				Page::AlbumList => album_list::update(album_list::Msg::Show, &mut model.album_list, &mut orders.proxy(Msg::AlbumList)),
				_ => (),
			};
		},
        Msg::Login(msg) => {
            match msg {
                login::Msg::SetIsAuth => {
					model.is_auth = true;
					model.page = Page::Menu;
					orders.send_msg(Msg::SetUrl);
					orders.send_msg(Msg::LoadPage);
					header::update(header::Msg::Show, &mut model.header, &mut orders.proxy(Msg::Header));
				}
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
		Msg::Menu(msg) => {
			menu::update(msg, &mut model.menu, &mut orders.proxy(Msg::Menu));
		},
		Msg::Header(msg) => {
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
        Msg::ShowToast(toast) => {
            toast::update(toast::Msg::Show(toast), &mut model.toast, &mut orders.proxy(Msg::Toast));
        },
        Msg::AlbumList(msg) => {
			album_list::update(msg, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
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
	let s_main = style! {
		St::Height => percent(100),
		St::Background => "radial-gradient(circle at bottom right, #0f3057 -20%, #d0fcff 100%)",
	};

    div![style,
		toast::view(&model.toast).map_msg(Msg::Toast),
		
		match &model.page {
			Page::Login => login::view(&model.login).map_msg(Msg::Login),
			_ => nodes![
					match model.is_auth {
						true => div![
							s_main,
							header::view(&model.header).map_msg(Msg::Header),
							match &model.page {
								Page::Menu => menu::view(&model.menu).map_msg(Msg::Menu),
								Page::AlbumList => album_list::view(&model.album_list).map_msg(Msg::AlbumList),
								_ => nodes![],
							}
						],
						false => empty![],
					}
				]	
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
