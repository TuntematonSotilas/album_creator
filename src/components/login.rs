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
    nodes![
        match model.is_auth {
            false => form![
                ev(Ev::Submit, |event| {
                    event.prevent_default();
                    Msg::Submit
                }),
                label![
                    "Login",
                    input![
                        attrs! {At::Value => model.login},
                        input_ev(Ev::Input, Msg::NameChanged),
                    ]
                ],
                label![
                    "Password",
                    input![
                        attrs! {At::Value => model.pwd, At::Type => "password"},
                        input_ev(Ev::Input, Msg::PwdChanged),
                    ]
                ],
                button!["Submit"],
                if let Some(message) = &model.message {
                    span![message]
                } else {
                    empty![]
                },
            ],
            true => empty![],
        }
    ]
}