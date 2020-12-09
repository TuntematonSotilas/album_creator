use seed::{self, prelude::*, *};

use crate::models::toast::Toast;
use crate::utils::{
    style::s_button,
    vars::{LOGIN, PWD}
};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    login: String,
	pwd: String,
}

// ------------
//    Update
// ------------

pub enum Msg {
    Submit,
    NameChanged(String),
    PwdChanged(String),
    SetIsAuth,
    ShowToast(Toast),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(login) => model.login = login,
        Msg::PwdChanged(pwd) => model.pwd = pwd,
        Msg::Submit => {
            if model.login == LOGIN && model.pwd == PWD {
                orders.send_msg(Msg::SetIsAuth);
                orders.send_msg(Msg::ShowToast(
                    Toast {
                        is_visible: false,
                        title: None,
                        content: None,
                    }));
            } else {
                orders.send_msg(Msg::ShowToast(
                    Toast { 
                        is_visible: true,
                        title: Some("Login failed !".into()),
                        content: Some("Try again.".into()),
                    }));
            }
        },
        Msg::SetIsAuth => (),
        Msg::ShowToast(_toast) => (),
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_column = style! {
        St::Display => "flex",
        St::FlexDirection => "column",
    };
    let s_login = style! {
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::Background => "radial-gradient(circle at bottom left, rgba(130, 130, 130, 0.5) -10%, rgba(0,0,0,0) 100%), 
            radial-gradient(circle at top left, #008891 -20%, #0f3057 100%)",
        St::Height => percent(100),
        St::Color => "white",
        St::MinHeight => vh(100),
    };
    let s_form = style! {
        St::Width => rem(12),
    };
    let s_h1 = style! {
        St::FontSize => rem(2),
        St::Margin => 0,
    };
    let s_h2 = style! {
        St::FontSize => rem(0.7),
        St::Color => "rgba(255,255,255,0.5)",
        St::MarginTop => em(0.1),
        St::MarginBottom => em(2),
    };
    let s_titles = style! {
        St::LetterSpacing => rem(0.1),
        St::TextAlign => "center"
        St::TextShadow => "0 0 1rem rgba(0,0,0,0.3)",
    };
    let s_input = style! {
        St::Padding => rem(0.5),
        St::MarginBottom => rem(0.5),
        St::Background => "rgba(0, 0, 0, 0.3)",
        St::Outline => "none",
        St::Color => "white",
        St::Border => "1px solid rgba(0,0,0,0.3)",
        St::BorderRadius => rem(0.3),
        St::Transition => "box-shadow .5s ease",
        St::TextShadow => "1px 1px 1px rgba(0,0,0,0.3)",
    };
    nodes![
        div![
            &s_column,
            s_login,
            form![
                ev(Ev::Submit, |event| {
                    event.prevent_default();
                    Msg::Submit
                }),
                div![
                    &s_column,
                    s_form,
                    h1![
                        s_h1,
                        &s_titles,
                        "Login"
                    ],
                    h2![
                        s_h2,
                        &s_titles,
                        "Album Creator"
                    ],
                    input![
                        C!("login__input"),
                        &s_input,
                        attrs! {
                            At::Value => model.login,
                            At::Placeholder => "Username",
                        },
                        input_ev(Ev::Input, Msg::NameChanged),
                    ],
                    input![
                        C!("login__input"),
                        &s_input,
                        attrs! {
                            At::Value => model.pwd, 
                            At::Type => "password"
                            At::Placeholder => "Password",
                        },
                        input_ev(Ev::Input, Msg::PwdChanged),
                    ],
                    button![
                        C!("button"),
                        s_button(),
                        "Submit"
                    ],
                ],
            ],
        ]
    ]
}