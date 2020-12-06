use seed::{self, prelude::*, *};
use web_sys::FileList;

use crate::{
	models::picture::Picture,
	utils::{
		style::s_button, 
		vars::API_URI, 
		request::get_auth,
		deserializer::deser_upload_resp,
		serializer::ser_new_picture,
	}
};


// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	show: bool,
	album_id: String,
	order: i32,
	picture: Option<Picture>,
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show(String, i32),
	FilesChanged(Option<FileList>),
	SetPicData(Option<String>),
	Post,
	Result(Option<String>),
	SetUploadResult(Option<Picture>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(album_id, order) => {
			model.show = true;
			model.album_id = album_id;
			model.order = order;
		},
		Msg::FilesChanged(files_opt) => {
			if let Some(files) = files_opt {
				let file_opt = files.get(0);
				if let Some(file) = file_opt {
					orders.perform_cmd(async move {
						let mut data_opt: Option<String> = None;
						let data_res = gloo_file::futures::read_as_data_url(&gloo_file::Blob::from(file.clone())).await;
						if let Ok(data) = data_res {
							data_opt = Some(data);
						}
						Msg::SetPicData(data_opt)
					});
				}
			}
		},
		Msg::SetPicData(data_opt) => {
			if let Some(data) = data_opt {
				model.picture = Some(
					Picture {
						id: None,
						album_id: model.album_id.clone(),
						order: model.order,
						data: data,
					}
				);
				orders.send_msg(Msg::Post);
			}	
		}
		Msg::Post => {
			orders.skip(); // No need to rerender

			if let Some(picture) = model.picture.clone() {
				let uri = format!("{0}new-picture", API_URI);
				let request = Request::new(uri)
					.method(Method::Post)
					.header(Header::authorization(get_auth()))
					.json(&ser_new_picture(picture));
				
				orders.perform_cmd(async {
					let mut id_opt: Option<String> = None;
					if let Ok(json) = request {
						let result = fetch(json).await;
						id_opt = deser_upload_resp(result).await;
					}
					Msg::Result(id_opt)
				});
			}
		},
		Msg::Result(id_opt, ) => {
			if let Some(picture) = &mut model.picture {
				picture.id = id_opt;
				orders.send_msg(Msg::SetUploadResult(model.picture.clone())); 
			}
		},
		Msg::SetUploadResult(_pic_opt) => {},
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let s_input_file = style! {
		St::Display => "none",
	};
	let s_btn = style! {
		St::MarginTop => rem(1),
	};
	nodes![
		IF!(model.show => 
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
			]
		)
	]
}