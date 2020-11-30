use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {}

// ------------
//    Update
// ------------

pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

// ------------
//     View
// ------------

pub fn view(_model: &Model) -> Vec<Node<Msg>> {
    let s_file_btn = style! {
		St::Width => rem(5),
        St::Height => rem(5),
		St::Margin => rem(1),
		St::BorderRadius => rem(0.2),
		St::FontSize => rem(2),
		St::TextAlign => "center",
		St::LineHeight => rem(5),
		St::Color => "#3f46ac",
		St::Background => "rgba(0, 0, 0, 0.2)",
		St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
	};
	let s_input_file = style! {
		St::Display => "none",
	};
	nodes![
		label![
            s_file_btn,
            C!("edit_ablum__file_btn"),
            i![
                C!("fa fa-plus"),
            ],
            input![
                s_input_file,
                attrs! {
                    At::Type => "file",
                    At::Accept => "image/*",
                },
            ],
        ],
	]
}