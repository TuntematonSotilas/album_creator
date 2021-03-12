use seed::{prelude::*, *};

use crate::components::{
    login,
    toast,
	menu,
    header,
	album_list,
	edit_album,
	album,
	confirm,
};
use crate::models::toast::Toast;
use crate::utils::pubvars::SWITCH_TIMEOUT;

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
        menu: menu::Model::new(&url),
		header: header::Model::new(&url),
		album_list: album_list::Model::new(&url),
		edit_album: edit_album::Model::default(),
		album: album::Model::default(),
		confirm: confirm::Model::default(),
        base_url: url.to_base_url(),
        page: Page::init(url),
		is_auth: false,
		id_url: None,
		is_edit: false,
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
	edit_album: edit_album::Model,
	album: album::Model,
    confirm: confirm::Model,
	base_url: Url,
    page: Page,
	is_auth: bool,
	id_url: Option<String>,
	is_edit: bool
}

#[derive(Copy, Clone)]
pub enum Page {
    Login,
    Menu,
	AlbumList,
	EditAlbum,
	Album,
}

impl Default for Page {
    fn default() -> Self {
        Page::Login
    }
}

impl Page {
    fn init(mut url: Url) -> Self {
		match url.next_path_part() {
            None => Self::Login,
            Some("menu") => Self::Menu,
			Some("albums") => Self::AlbumList,
			Some("edit") => Self::EditAlbum,
			Some("album") => Self::Album,
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
	EditAlbum(edit_album::Msg),
	Album(album::Msg),
	Confirm(confirm::Msg),
    ShowToast(Toast),
	UrlChanged(subs::UrlChanged),
	SetUrl(Option<String>),
	LoadPage,
	ShowConfirm(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
			if !model.is_auth {
				model.page = Page::Login;
				orders.send_msg(Msg::SetUrl(None));
			} else {
				model.page = Page::init(url.clone());
			}
			let mut url_cp = url.clone(); 
			url_cp.next_path_part();
			if let Some(id_url) = url_cp.next_path_part() {
				model.id_url = Some(id_url.into());
			}
			orders.send_msg(Msg::LoadPage);
		},
		Msg::SetUrl(id_opt) => {
			let path_part = match model.page {
				Page::Menu => "menu",
				Page::AlbumList => "albums",
				Page::Album => "album",
				Page::EditAlbum => "edit",
				_ => "login",
			};
			let mut url = model.base_url.clone().add_path_part(path_part);
			if let Some(id) = id_opt {
				url = url.clone().add_path_part(id);
			}
			url.go_and_push();
		},
		Msg::LoadPage => {
			model.is_edit = match model.page {
				Page::EditAlbum => true,
				_ => false,
			};
			match model.page {
				Page::Menu => menu::update(menu::Msg::Show, &mut model.menu, &mut orders.proxy(Msg::Menu)),
				Page::AlbumList => album_list::update(album_list::Msg::Show, &mut model.album_list, &mut orders.proxy(Msg::AlbumList)),
				Page::Album => album::update(album::Msg::Show(model.id_url.clone()), &mut model.album, &mut orders.proxy(Msg::Album)),
				Page::EditAlbum => edit_album::update(edit_album::Msg::Show(model.id_url.clone()), &mut model.edit_album, &mut orders.proxy(Msg::EditAlbum)),
				_ => (),
			};
		},
        Msg::Login(msg) => {
            match msg {
                login::Msg::SetIsAuth => {
					model.is_auth = true;
					model.page = Page::Menu;
					orders.send_msg(Msg::SetUrl(None));
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
			match msg {
				album_list::Msg::ShowConfirm(ref message) => {
					orders.send_msg(Msg::ShowConfirm(message.clone()));
				},
				_ => (),
			}
			album_list::update(msg, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
        },
        Msg::EditAlbum(msg) => {
			match msg {
				edit_album::Msg::ShowConfirm(ref message) => {
					orders.send_msg(Msg::ShowConfirm(message.clone()));
				},
				edit_album::Msg::GoToConsult(ref frid) => {
					model.page = Page::Album;
					orders.send_msg(Msg::SetUrl(Some(frid.into())));
					orders.send_msg(Msg::LoadPage);
				},
				edit_album::Msg::AnimBckg(is_edit) => {
					model.is_edit = is_edit;
				},
				_ => (),
			}
			edit_album::update(msg, &mut model.edit_album, &mut orders.proxy(Msg::EditAlbum));
		},
		Msg::Album(msg) => {
			match msg {
				album::Msg::GoToEdit(ref frid) => {
					model.page = Page::EditAlbum;
					orders.send_msg(Msg::SetUrl(Some(frid.into())));
					orders.send_msg(Msg::LoadPage);
				},
				album::Msg::AnimBckg(is_edit) => {
					model.is_edit = is_edit;
				},
				_ => (),
			}
			album::update(msg, &mut model.album, &mut orders.proxy(Msg::Album));
		},
		Msg::Confirm(msg) => {
			match msg {
				confirm::Msg::Ok => {
					match model.page {
						Page::AlbumList => {
							album_list::update(album_list::Msg::Delete, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
						},
						Page::EditAlbum => {
							edit_album::update(edit_album::Msg::DeletePic, &mut model.edit_album, &mut orders.proxy(Msg::EditAlbum));
						},
						_ => (),
					}
				},
				_ => (),
			}
			confirm::update(msg, &mut model.confirm, &mut orders.proxy(Msg::Confirm));
		},
		Msg::ShowConfirm(message) => {
			confirm::update(confirm::Msg::Show(message), &mut model.confirm, &mut orders.proxy(Msg::Confirm));
		}
    }
}

// ------------
//     View
// ------------

fn view(model: &Model) -> Node<Msg> {
    let style = style! { 
        St::MinHeight => vh(100),
        St::FontFamily => "'Open Sans', sans-serif",
	};
	let s_main = style! {
		St::MinHeight => vh(100),
	};
	let s_bckg = style! {
		St::Transition => format!("background {0}ms linear", SWITCH_TIMEOUT),
		St::Background => match model.is_edit {
			true => "#fc8621",
			false => "#264871",
		},
		St::BackgroundImage => "radial-gradient(circle at top left, rgba(255, 255, 255, 0.6), rgba(0, 0, 0, 0) 120%)";
	};
    div![
		style,
		toast::view(&model.toast).map_msg(Msg::Toast),
		confirm::view(&model.confirm).map_msg(Msg::Confirm),
		match &model.page {
			Page::Login => login::view(&model.login).map_msg(Msg::Login),
			_ => nodes![
					match model.is_auth {
						true => div![
							s_main,
							s_bckg,
							header::view(&model.header).map_msg(Msg::Header),
							match &model.page {
								Page::Menu => menu::view(&model.menu).map_msg(Msg::Menu),
								Page::AlbumList => album_list::view(&model.album_list).map_msg(Msg::AlbumList),
								Page::EditAlbum => edit_album::view(&model.edit_album).map_msg(Msg::EditAlbum),
								Page::Album => album::view(&model.album).map_msg(Msg::Album),
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
