use seed::{self, prelude::*, *};

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