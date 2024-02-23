mod auth_flow_models;
mod auth_flow;

use websocket::ClientBuilder;
use websocket::sync::Client;
use websocket::stream::sync::NetworkStream;
use serde::{Serialize, Deserialize};
use serde;
use crate::app::auth_flow_models::AvailableAuthTypes;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherApp {
    login: String,
    password: String,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    ws_client: Client<Box<dyn NetworkStream + Send>>,
}

impl Default for LauncherApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            login: String::from("login"),
            password: String::from("password"),
            // ws_client: ClientBuilder::new("").unwrap().connect(None).unwrap()
            ws_client: match ClientBuilder::new(auth_flow::SERVER_WSS_URL) {
                Ok(mut res) => {
                    match res.connect(None) {
                        Ok(connect_res) => { connect_res }
                        Err(_) => {
                            std::process::exit(-1);
                        }
                    }
                }
                Err(_) => {
                    std::process::exit(-1);
                }
            },
        }
    }
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { login, password, ws_client } = self;

        egui::CentralPanel::default().show(ctx, |panel| {
            if panel.button("x").clicked() {
                _frame.close()
            };
            panel.vertical_centered(|it| {
                it.heading("Dreamfinity Rust Launcher");
            });

            panel.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.text_edit_singleline(login);
                    ui.text_edit_singleline(password);
                    if ui.button("Log in").clicked() {
                        let auth_types = auth_flow::get_auth_types(ws_client);
                        match auth_types {
                            None => {}
                            Some(res) => for auth_type in res.list {
                                println!("{}", auth_type.displayName);
                            },
                        }
                    }
                    ui.hyperlink("https://dreamfinity.org");
                });
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
