use seed::{self, prelude::*, *};

// ------------
//     Model
// -----------

#[derive(Default)]
pub struct Model {
    is_auth: bool,
    message: Option<String>,
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
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(login) => model.login = login,
        Msg::PwdChanged(pwd) => model.pwd = pwd,
        Msg::Submit => {
            /*if model.login == LOGIN && model.pwd == PWD {
                model.is_auth = true;
                model.message = None;
            } else {
                model.message = Some("Login failed".into());
            }*/
        }
    }
}

// ------------
//     View
// ------------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        form![
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
                    attrs! {At::Value => model.pwd},
                    input_ev(Ev::Input, Msg::PwdChanged),
                ]
            ],
            button!["Submit"],
            if let Some(message) = &model.message {
                span![message]
            } else {
                empty![]
            },
        ]
    ]
}