use seed::{self, prelude::*, *};

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

pub fn s_btn_icon(size: Size) -> Style {
	let size = match size {
		Size::S => 2.5,
		Size::X => 5.
	};
	style! {
		St::Width => rem(size),
		St::Width => rem(size),
		St::LineHeight => rem(size),
        St::Margin => rem(1),
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