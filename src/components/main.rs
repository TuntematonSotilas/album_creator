use seed::{self, prelude::*, *};

use crate::components::{menu, header, album_list};

// ------------
//     Model
// -----------

#[derive(Copy, Clone)]
pub enum Page {
	AlbumList,
	NewAlbum,
	Menu,
 }

 #[derive(Default)]
pub struct Model {
	is_auth: bool,
	page: Option<Page>,
	menu: menu::Model,
	header: header::Model,
	album_list: album_list::Model, 
}

impl Model {
	pub fn new() -> Model {
		Model {
			is_auth: false,
			page: Some(Page::Menu),
			menu: menu::Model::default(),
			header: header::Model::default(),
			album_list: album_list::Model::default(),
		}
	}
}

// ------------
//    Update
// ------------

pub enum Msg {
    SetIsAuth(bool),
	Menu(menu::Msg),
	Header(header::Msg),
	AlbumList(album_list::Msg),
	ShowPage(Page),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetIsAuth(is_auth) => {
			model.is_auth = is_auth;
			menu::update(menu::Msg::Show, &mut model.menu, &mut orders.proxy(Msg::Menu));
		},
        Msg::Menu(msg) => {
			match msg {
				menu::Msg::ShowPage(page) => {
					orders.send_msg(Msg::ShowPage(page));
				}
				_ => (),
            };
            menu::update(msg, &mut model.menu, &mut orders.proxy(Msg::Menu));
		},
		Msg::Header(msg) => {
			match msg {
				header::Msg::ShowPage(page) => {
					orders.send_msg(Msg::ShowPage(page));
				}
            };
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
		Msg::AlbumList(msg) => {
			album_list::update(msg, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
		},
		Msg::ShowPage(page) => {
			model.page = Some(page);
			match page {
				Page::AlbumList => {
					album_list::update(album_list::Msg::Fetch, &mut model.album_list, &mut orders.proxy(Msg::AlbumList));
				},
				Page::Menu => {
					menu::update(menu::Msg::Show, &mut model.menu, &mut orders.proxy(Msg::Menu));
				},
				_ => (),
			};
		},
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_main = style! {
		St::Height => percent(100),
		St::Background => "radial-gradient(circle at top left, #8bd2d6 -20%, #9bbade 100%)",
	};
    nodes![
		IF!(model.is_auth => 
        	div![
				s_main,
				header::view(&model.header).map_msg(Msg::Header),
				IF!(model.page.is_some() => 
					match model.page.unwrap() {
						Page::Menu => menu::view(&model.menu).map_msg(Msg::Menu),
						Page::AlbumList => album_list::view(&model.album_list).map_msg(Msg::AlbumList),
						_ => nodes![],
					}
				),
			]
		),
    ]
}