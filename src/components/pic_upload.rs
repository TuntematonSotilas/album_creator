use seed::{self, prelude::*, *};
use web_sys::{FileList, FileReader, Blob};

use crate::utils::style::s_button;


// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

// ------------
//    Update
// ------------

pub enum Msg {
	FilesChanged(Option<FileList>),
}

pub fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::FilesChanged(files_opt) => {
			if let Some(files) = files_opt {
				let file_opt = files.get(0);
				if let Some(file) = file_opt {
					orders.perform_cmd(async move {
						let data_res = gloo_file::futures::read_as_data_url(&gloo_file::Blob::from(file.clone())).await;
						if let Ok(data) = data_res {
							log!(data);
						}
					});
					
				}
			}
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