use seed::{self, prelude::*, *};

use friendly_id;

use crate::{
	components::pic_upload,
	utils::{
		vars::API_URI, 
		request::{get_auth, get_album, get_picture},
		serializer::{ser_edit_album, ser_edit_picture},
		style::*, 
		pubvars::{MAX_LOAD, SWITCH_TIMEOUT},
	},
	models::{album::Album, picture},
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
	album: Album,
	status: Status,
	pic_upload: pic_upload::Model,
	loaded: usize,
	is_switched: bool,
	pic_id_to_delete: Option<String>,
}

#[derive(PartialEq)]
pub enum Status {
	New,
	Edited,
	Saving,
	Saved,
	Error,
}

impl Default for Status {
    fn default() -> Self {
        Status::New
    }
}

// ------------
//    Update
// ------------

pub enum Msg {
	Show(Option<String>),
	AlbumRecieved(Option<Album>),
	LoadPictures,
	NameChange(String),
	Post,
	SetStatus(Status),
	PicUpload(pic_upload::Msg),
	CaptionChange(Option<String>, String),
	EditPicture(picture::Picture),
	GetPicture(String),
	PictureReceived(Option<String>, String),
	Save,
	Switch,
	GoToConsult(String),
	AnimBckg(bool),
	AskDeletePic(Option<String>),
	DeletePic,
	DeletePicResp(bool),
	ShowConfirm(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(opt_id) => {
			model.status = Status::New;
			model.loaded = 0;
			model.is_switched = true;
			model.album = Album::default();
			match opt_id {
				Some(id) => {
					orders.perform_cmd(async {
						let album_opt = get_album(id).await;
						Msg::AlbumRecieved(album_opt)
					});
				},
				None => {
					model.album.frid = friendly_id::create();
					model.album.name = String::new();
					model.album.pictures = Vec::new();
				}
			}
		},
		Msg::NameChange(name) => {
			model.album.name = name;
			match model.status {
				Status::New => (),
				_ => { orders.send_msg(Msg::SetStatus(Status::Edited)); }
			} 
		},
		Msg::AlbumRecieved(opt) => {
			orders.send_msg(Msg::SetStatus(Status::Saved));
			if let Some(album) = opt {
				model.album = album;
				orders.send_msg(Msg::LoadPictures);
			}
		},
		Msg::LoadPictures => {
			//Load only X pictures in DOM 
			model.album.pictures.iter_mut()
				.filter(|p| !p.dom)
				.take(MAX_LOAD)
				.for_each(|p| p.dom = true);
			model.loaded += MAX_LOAD;
			//Load pictures
			model.album.pictures.iter()
				.filter(|p| p.dom && p.id.is_some())
				.for_each(|p| {
					orders.send_msg(Msg::GetPicture(p.id.clone().unwrap()));
				});
			
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
			model.album.pictures.iter_mut()
				.find(|p| p.id == Some(id.clone()))
				.map(|p| p.data = data);
		},
		Msg::Post => {
			orders.skip(); // No need to rerender
			
			model.status = Status::Saving;

			let uri = format!("{0}edit-album", API_URI);
			let request = Request::new(uri)
				.method(Method::Post)
				.header(Header::authorization(get_auth()))
				.json(&ser_edit_album(model.album.clone()));
			
			orders.perform_cmd(async {
				let mut status = Status::Error;
				if let Ok(json) = request {
					let result = fetch(json).await;
					if let Ok(response) = result {
						if let Ok(_) = response.check_status() {
							status = Status::Saved;
						}
					}
				}
				Msg::SetStatus(status)
			});
		},
		Msg::SetStatus(status) => {
			model.status = status;
			match model.status {
				Status::Saved => {
					//Set all pics as saved
					model.album.pictures.iter_mut()
						.for_each(|p| p.saved = true);
					//Update upload component 
					let max = model.album.pictures.iter().map(|p| p.order).max();
					pic_upload::update(pic_upload::Msg::Show(
						model.album.frid.clone(), max.unwrap_or(0)), 
						&mut model.pic_upload, 
						&mut orders.proxy(Msg::PicUpload));
				},
				_ => (),
			}
		},
		Msg::PicUpload(msg) => {
			match msg {
				pic_upload::Msg::BeginUpload(ref pic_opt) => {
					if let Some(picture) = pic_opt.clone() {
						model.album.pictures.push(picture);
					}
				},
				pic_upload::Msg::EndUpload(ref id_opt) => {
					model.album.pictures.iter_mut()
						.filter(|p| p.id == None)
						.for_each(|p| p.id = id_opt.clone());
				},
				_ => ()
			}
			pic_upload::update(msg, &mut model.pic_upload, &mut orders.proxy(Msg::PicUpload));
		},
		Msg::CaptionChange(pic_id, caption) => {
			orders.send_msg(Msg::SetStatus(Status::Edited));
			model.album.pictures.iter_mut()
				.filter(|p| p.id == pic_id)
				.for_each(|p| {
					p.caption = Some(caption.clone());
					p.saved = false;
				});
		},
		Msg::EditPicture(picture) => {
			orders.skip(); // No need to rerender

			model.status = Status::Saving;

			let uri = format!("{0}edit-picture", API_URI);
			let request = Request::new(uri)
				.method(Method::Post)
				.header(Header::authorization(get_auth()))
				.json(&ser_edit_picture(picture.clone()));
			
			orders.perform_cmd(async {
				let mut status = Status::Error;
				if let Ok(json) = request {
					let result = fetch(json).await;
					if let Ok(response) = result {
						if response.check_status().is_ok() {
							status = Status::Saved;
						}
					}
				}
				Msg::SetStatus(status)
			});
		},
		Msg::Save => {
			orders.send_msg(Msg::Post);
			//Update pictures not saved
			model.album.pictures.iter()
				.filter(|p| !p.saved)
				.for_each(|p| {
					orders.send_msg(Msg::EditPicture(p.clone()));
				});
		},
		Msg::Switch => {
			orders.send_msg(Msg::AnimBckg(false));
			model.is_switched = false;
			let frid = model.album.frid.clone();
			orders.perform_cmd(cmds::timeout(SWITCH_TIMEOUT, ||Msg::GoToConsult(frid)));
		},
		Msg::GoToConsult(_frid) => (),
		Msg::AnimBckg(_is_edit) => (),
		Msg::AskDeletePic(id) => {
			model.pic_id_to_delete = id;
			orders.send_msg(Msg::ShowConfirm("Delete picture ?".into()));
		},
		Msg::DeletePic => {
			orders.skip(); // No need to rerender
			if let Some(id) = &model.pic_id_to_delete {	
				let uri = format!("{0}delete-picture?id={1}", API_URI, id);	
				let mut is_ok = false;
				orders.perform_cmd(async move {
					let request = Request::new(uri)
						.method(Method::Delete)
						.header(Header::authorization(get_auth()));
					let result = fetch(request).await;
					if let Ok(response) = result {
						if response.check_status().is_ok() {
							is_ok = true;
						}
					}
					Msg::DeletePicResp(is_ok)
				});
			}
		},
		Msg::DeletePicResp(is_ok) => {
			if is_ok {
				let position = &model.album.pictures.iter()
					.position(|a| a.id == model.pic_id_to_delete);
				if let Some(pos) = position {
					model.album.pictures.remove(*pos);
				}
			}
		},
		Msg::ShowConfirm(_msg) => (),
	}
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	let (status, color): (String, String) = match model.status {
		Status::New => (
			"NEW".into(),
			"#0396ff".into()),
		Status::Edited => (
			"EDITED".into(),
			"#0396ff".into()),
		Status::Saving => (
			"SAVING".into(),
			"#788d95".into()),
		Status::Saved => (
			"SAVED".into(), 
			"#0ad406".into()),
		Status::Error => (
			"ERROR".into(), 
			"#794242".into()),
	};
	let s_main = style! {
		St::Display => "flex",
		St::FlexDirection => "column",
		St::AlignItems => "center",
	};
	let s_status = style! {
		St::Color => color,
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::FontSize => rem(0.8),
		St::LetterSpacing => rem(0.2),
		St::TransitionProperty => "color",
		St::TransitionDuration => "200ms",
		St::AlignSelf => "center",
		St::FontWeight => "bold",
	};
	let s_header = style! {
		St::Display => "flex",
		St::Width => vw(90),
		St::Height => rem(5),
		St::AlignItems => "center",
	};
	let s_header_left = style! {
		St::Width => vw(85),
		St::Display => "flex",
	};
	let s_panel = style! {
		St::AlignItems => "center",
		St::Display => "flex",
		St::JustifyContent => "space-between"
		St::Background => "radial-gradient(circle at bottom left, rgba(28, 28, 36, 0.5) 40%, transparent 120%)",
		St::BorderRadius => rem(0.3),
		St::Padding => rem(0.5),
		St::Width => vw(90),
		St::MarginBottom => rem(1),
	};
	let s_panel_name = style! {
		St::FlexDirection => "column",
	};
	let s_label = style! {
		St::Position => "relative",
		St::Top => rem(-1.4),
		St::Color => "rgba(255,255,255,0.6)",
		St::Transition => "all 0.2s ease",
		St::PointerEvents => "none",
	};
	let s_input = style! {
		St::Display => "block",
		St::Width => percent(100),
		St::MarginTop => rem(1),
		St::Outline => "none",
		St::Background => "none",
		St::Color => "white",
		St::LetterSpacing => rem(0.1),
		St::BorderBottom => "solid 1px white",
		St::BorderTop => "none",
		St::BorderLeft => "none",
		St::BorderRight => "none",
		St::FontSize => rem(1.5),
	};
	let s_name_input = style! {
		St::FontSize => rem(1.5),
	};
	let s_list = style! {
		St::ListStyle => "none",
		St::Padding => 0,
		St::Display => "flex",
		St::FlexDirection => "column",
	};
	let s_caption = style! {
		St::MarginLeft => rem(2),
	};
	let s_pic_border = style! {
		St::Margin => rem(0.5),
		St::BorderRadius => rem(0.2),
	};
	let s_pic_img = style! {
		St::MaxWidth => rem(6),
	};
	let s_pic_empty = style! {
		St::Width => rem(5),
		St::Height => rem(5),
		St::Background => "rgba(0, 0, 0, 0.2)",
	};
	let s_delete = style! {
		St::FontSize => rem(0.8),
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
		St::AlignSelf => "flex-start",
	};
	nodes![
		div![
			s_main,
			div![
				s_header,
				div![
					s_header_left,
					a![
						C!("btn_icon btn_icon--blue"),
						s_btn_icon(Size::S, 1),
						ev(Ev::Click, |_| Msg::Save),
						i![
							C!("fa fa-check"),
						],
						attrs! { At::Href => String::new()},
					],
					span![
						s_status,
						status
					],
				],
				IF!(model.status != Status::New =>  
					div![
						s_switch(),
						s_switch_anim(model.is_switched),
						span![
							s_switch_btn(), 
							s_switch_btn_anim(model.is_switched),
						],
						ev(Ev::Click, |_| Msg::Switch),
					]
				)
			],
			div![
				&s_panel,
				&s_panel_name,
				div![
					input![
						C!("edit_album__input"),
						&s_input,
						s_name_input,
						attrs! {
							At::Required => true,
							At::Value => model.album.name,
							At::MaxLength => 20,
						},
						input_ev(Ev::Input, Msg::NameChange),
					],
					label![
						&s_label,
						"Name",
					],
				],
			],
			ul![
				s_list,
				model.album.pictures.iter().filter(|p| p.dom).map(|pic| {
					let mut caption = String::new();
					if let Some(cap) = pic.caption.clone() {
						caption = cap;
					};
					let id = pic.id.clone();
					let id_del = pic.id.clone();
					li![
						&s_panel,
						match &pic.data {
							Some(data_url) => img![
								&s_pic_border,
								&s_pic_img,
								attrs!{ At::Src => data_url }
							],
							_ => div![
								&s_pic_border,
								&s_pic_empty,
								div![s_loader(), s_loader_1() ],
								div![s_loader(), s_loader_2() ],
							]
						},
						div![
							&s_caption,
							input![
								C!("edit_album__input"),
								&s_input,
								attrs! {
									At::Value => caption,
									At::Required => true,
									At::Disabled => pic.id.is_none().as_at_value(),
								},
								input_ev(Ev::Input, |value| Msg::CaptionChange(id, value)),
							],
							label![
								&s_label,
								"Caption",
							],
						],
						div![
							C!("delete_link"),
							&s_delete,
							"delete",
							ev(Ev::Click, |_| Msg::AskDeletePic(id_del)),
						],
					]
				}),
			],
			pic_upload::view(&model.pic_upload).map_msg(Msg::PicUpload),
		],
	]
}