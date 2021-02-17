use seed::{self, prelude::*, *};

use crate::utils::pubvars::SWITCH_TIMEOUT;

pub enum Size {
	S, X
}

pub fn s_button() -> Style {
    style! {
        St::Padding => rem(0.5),
        St::BackgroundImage => "linear-gradient(#6eb6de, #4a77d4)",
        St::BackgroundRepeat => "repeat-x",
        St::Border => "1px solid #3762bc",
        St::BorderRadius => rem(0.3),
        St::Color => "white",
        St::FontSize => rem(0.9),
        St::LetterSpacing => rem(0.1),
        St::TextShadow => "1px 1px 1px rgba(0,0,0,0.4)",
        St::Cursor => "pointer",
        St::Outline => "none",
        St::BoxShadow => "0 1px 2px rgba(0, 0, 0, 0.5)"
    }
}

pub fn s_btn_icon(size: Size, margin: u32) -> Style {
	let size = match size {
		Size::S => 2.5,
		Size::X => 5.
	};
	style! {
		St::Width => rem(size),
		St::LineHeight => rem(size),
        St::Margin => rem(margin),
        St::BorderRadius => rem(0.5),
        St::FontSize => rem(size / 3.),
        St::TextAlign => "center",
		St::BoxShadow => "0.2em 0.2em 0 0 rgba(0, 0, 0, 0.14)",
		St::Transition => "scale 200ms ease-out",
		St::TransitionTimingFunction => "cubic-bezier(0.2, 0.8, 0.3, 1.2)",
	}
}

pub fn s_loader() -> Style {
	style! {
		St::Position => "absolute",
		St::MarginLeft => rem(1),
		St::MarginTop => rem(1),
		St::Width => rem(3),
		St::Height => rem(3),
		St::Background => "rgba(0, 0, 0, 0.2)",
		St::BorderRadius => percent(50),
	}
}

pub fn s_loader_1() -> Style {
	style! {
		St::Transform => "scale(1)",
		St::Animation => "pulse 2s infinite linear",
	}
}

pub fn s_loader_2() -> Style {
	style! {
		St::Transform => "scale(0)",
		St::Animation => "pulse 2s 1s infinite linear",
	}
}

pub fn s_switch() -> Style {
	style! {
		St::Display => "flex",
		St::AlignItems => "center",
		St::Width => rem(3.3),
		St::Height => rem(1.5),
		St::AlignSelf => "center",
		St::BoxShadow => "inset 0px 0px 3px 0px rgba(0, 0, 0, 0.2), 0 1px 2px rgba(0, 0, 0, 0.5)",
		St::BorderRadius => rem(1),
		St::Cursor => "pointer",
		St::Transition => format!("background {0}ms ease", SWITCH_TIMEOUT),
	}
}

pub fn s_switch_btn() -> Style {
	style! {
		St::Background => "white",
		St::Width => rem(1.2),
		St::Height => rem(1.2),
		St::BorderRadius => rem(1),
		St::BoxShadow => "inset 0.2px -1px 1px rgba(0, 0, 0, 0.35)",
		St::Transition => format!("margin-left {0}ms ease", SWITCH_TIMEOUT),
	}
}

pub fn s_switch_anim(is_switched: bool) -> Style {
	match is_switched {
		true => style! { 
			St::Background => "#c24914",
		},
		false => style! { 
			St::Background => "#008891",
		}
	}
}

pub fn s_switch_btn_anim(is_switched: bool) -> Style {
	match is_switched {
		true => style! { 
			St::MarginLeft => rem(1.9),
		},
		false => style! { 
			St::MarginLeft => rem(0.1),
		}
	}
}