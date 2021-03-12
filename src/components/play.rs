use seed::{self, prelude::*, *};

use crate::{
	utils::{
        pubvars::MAX_LOAD,
        request::{get_album, get_picture},
        style::*,
    },
    models::album::Album,
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    loaded: usize,
    album: Option<Album>,
    curr_pic_index: usize,
}

// ------------
//    Update
// ------------

pub enum Msg {
    Show(Option<String>),
    AlbumRecieved(Option<Album>),
    LoadPictures,
    GetPicture(String),
	PictureReceived(Option<String>, String),
    NextPic,
    DoNextPic,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::Show(opt_id) => {
            model.album = None;
            model.curr_pic_index = 0;
            model.loaded = 0;
            orders.skip(); // No need to rerender
			let mut album_opt: Option<Album> = None;
			orders.perform_cmd(async {
				if let Some(id) = opt_id {
					album_opt = get_album(id).await;
				}
				Msg::AlbumRecieved(album_opt)
			});
        },
        Msg::AlbumRecieved(opt) => {
			model.album = opt;
            orders.send_msg(Msg::LoadPictures);
            orders.send_msg(Msg::NextPic);
		},
        Msg::LoadPictures => {
			if let Some(album) = &mut model.album {
				//Load only X pictures in DOM 
				album.pictures.iter_mut()
					.filter(|p| !p.dom)
					.take(MAX_LOAD)
					.for_each(|p| p.dom = true);
				model.loaded += MAX_LOAD;
				//Load pictures
				album.pictures.iter()
					.filter(|p| p.dom && p.id.is_some())
					.for_each(|p| {
						orders.send_msg(Msg::GetPicture(p.id.clone().unwrap()));
					});
			}
		},
        Msg::GetPicture(id) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
				let id_c = id.clone(); 
				let data_opt = get_picture(id_c).await;
				Msg::PictureReceived(data_opt, id)
			});
		},
        Msg::PictureReceived(data, id) => {
			if let Some(album) = &mut model.album {
				album.pictures.iter_mut()
					.find(|p| p.id == Some(id.clone()))
					.map(|p| p.data = data);
			}
		},
        Msg::NextPic => {
            orders.perform_cmd(cmds::timeout(2000, || Msg::DoNextPic));
        },
        Msg::DoNextPic => {
            model.curr_pic_index += 1;
            orders.send_msg(Msg::NextPic);
        },
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_main = style! {
        St::Display => "flex",
        St::JustifyContent => "center",
        St::AlignItems => "center",
        St::Height => "90vh",
    };
    let s_pic_ctn = style! {
        St::Display => "flex",
        St::Width => percent(100);
    };
    let s_pic = style! {
        St::Width => percent(100);
    };
    
	nodes![
        match &model.album {
			Some(album) => {
                let pic_opt = album.pictures.get(model.curr_pic_index);
                match  pic_opt {
                    Some(pic) => div![
                        s_main,
                        div![
                            &s_pic_ctn,
                            match &pic.data {
                                Some(data_url) => img![
                                    &s_pic,
                                    attrs!{ At::Src => data_url }
                                ],
                                None => div![
                                    div![s_loader(), s_loader_1() ],
                                    div![s_loader(), s_loader_2() ],
                                ],
                            }
                        ]
                    ],
                    None => empty![],
                }
            },
            None =>  empty![],
        },
	]
}