use seed::{self, prelude::*, *};
use web_sys::FileList;

use crate::utils::{style::s_button, vars::API_URI, request::get_auth};


// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	picture: Option<Picture>,
}

#[derive(serde::Serialize, Default)]
pub struct Picture {
	album_id: String,
	order: i32,
	caption: String,
	data: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
	FilesChanged(Option<FileList>),
	SetPic(Option<Picture>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::FilesChanged(files_opt) => {
			if let Some(files) = files_opt {
				let file_opt = files.get(0);
				if let Some(file) = file_opt {
					orders.perform_cmd(async move {
						let mut pic_opt: Option<Picture> = None;
						let data_res = gloo_file::futures::read_as_data_url(&gloo_file::Blob::from(file.clone())).await;
						if let Ok(data) = data_res {
							log!(data);
							let picture = Picture {
								album_id: "6aPtAy9t1bQmcAnAex2nLV".to_string(),
								order: 0,
								caption: "plop".to_string(),
								data: data,
							};
							pic_opt = Some(picture);
						}
						Msg::SetPic(pic_opt)
					});
					
				}
			}
		},
		Msg::SetPic(pic_opt) => {
			model.picture = pic_opt;
		}
	}
}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
	let s_input_file = style! {
		St::Display => "none",
	};
	let s_btn = style! {
		St::MarginTop => rem(1),
	};
	nodes![
		label![
			s_btn,
			s_button(),
			C!("button"),
            "Add picture",
            input![
                s_input_file,
                attrs! {
                    At::Type => "file",
					At::Accept => "image/*",
					At::Multiple => "false"
				},
				ev(Ev::Change, |event| {
                    let files = event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                        .and_then(|file_input| file_input.files());

                    Msg::FilesChanged(files)
                }),
            ],
        ],
	]
}