use seed::{self, prelude::*, *};
use std::collections::HashMap;

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    is_auth: bool,
    login: String,
    pwd: String,
    message: Option<String>,
    config: HashMap<String, String>,
}

impl Model {
    pub fn new(config: HashMap<String, String>) -> Self {
        Self {
            is_auth: false,
            login: String::new(),
            pwd: String::new(),
            message: None,
            config: config,
        }
    }
}

// ------------
//    Update
// ------------

pub enum Msg {
    Submit,
    NameChanged(String),
    PwdChanged(String),
    SetIsAuth(bool),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(login) => model.login = login,
        Msg::PwdChanged(pwd) => model.pwd = pwd,
        Msg::Submit => {
            let conf_login = model.config.get("LOGIN");
            let conf_pwd = model.config.get("PWD");
            if conf_login.is_some() && conf_pwd.is_some() {
                if model.login == *conf_login.unwrap() && model.pwd == *conf_pwd.unwrap() {
                    model.message = None;
                    orders.send_msg(Msg::SetIsAuth(true));
                } else {
                    model.message = Some("Login failed".into());
                }
            }
        },
        Msg::SetIsAuth(is_auth) => model.is_auth = is_auth,
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let s_login = style! {
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::Background => "radial-gradient(circle at bottom left, rgba(130, 130, 130, 0.5) -5%, rgba(0,0,0,0) 100%), 
            radial-gradient(circle at top left, #008891 -20%, #0f3057 100%)",
        St::Height => percent(100),
        St::Color => "#fff",
    };
    let s_column = style! {
        St::Display => "flex",
        St::FlexDirection => "column",
    };
    let s_title = style! {
        St::FontSize => em(2),
        St::Margin => em(0.67),
        St::LetterSpacing => rem(0.1),
        St::TextAlign => "center"
        St::TextShadow => "0 0 10px rgba(0,0,0,0.3)",
    };
    let s_input = style! {
        St::Padding => rem(0.5),
        St::Background => "rgba(0, 0, 0, 0.3)",
        St::Outline => "none",
        St::Color => "#fff",
        St::Border => "1px solid rgba(0,0,0,0.3)",
        St::BorderRadius => rem(0.3),
        St::BoxShadow => "inset 0 -5px 45px rgba(100,100,100,0.2), 0 1px 1px rgba(255,255,255,0.2)",
    };
    nodes![
        match model.is_auth {
            false => div![
                s_column.clone(),
                s_login,
                form![
                    div![
                        s_column.clone(),
                        h1![
                            s_title,
                            "Login"
                        ],
                        input![
                            s_input,
                            attrs! {
                                At::Value => model.login,
                                At::Placeholder => "Username",
                            },
                            input_ev(Ev::Input, Msg::NameChanged),
                        ],
                        input![
                            attrs! {
                                At::Value => model.pwd, 
                                At::Type => "password"
                                At::Placeholder => "Password",
                            },
                            input_ev(Ev::Input, Msg::PwdChanged),
                        ],
                        button!["Submit"],
                        if let Some(message) = &model.message {
                            span![message]
                        } else {
                            empty![]
                        }, 
                    ],
                    ev(Ev::Submit, |event| {
                        event.prevent_default();
                        Msg::Submit
                    }),
                ],
            ],
            true => empty![],
        }
    ]
}