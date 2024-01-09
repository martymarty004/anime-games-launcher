use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

pub mod check_default_dirs;
pub mod init_config;
pub mod update_integrations;
pub mod init_games;
pub mod check_wine;
pub mod check_dxvk;
pub mod check_wine_prefix;
pub mod check_addons;
pub mod load_app;

use crate::APP_ID;

use super::main::MainApp;

pub static mut WINDOW: Option<adw::ApplicationWindow> = None;
pub static mut MAIN_WINDOW: Option<AsyncController<MainApp>> = None;

#[derive(Debug)]
pub struct LoadingApp {
    pub progress_bar: gtk::ProgressBar,

    pub active_stage: String
}

#[derive(Debug, Clone)]
pub enum LoadingAppMsg {
    SetActiveStage(String),
    SetProgress(f64),

    DisplayError {
        title: String,
        message: String
    }
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for LoadingApp {
    type Init = ();
    type Input = LoadingAppMsg;
    type Output = ();

    view! {
        window = adw::ApplicationWindow {
            set_default_size: (700, 520),
            set_title: Some("Anime Games Launcher"),

            set_resizable: false,
            // set_hide_on_close: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    add_css_class: "flat"
                },

                adw::StatusPage {
                    set_vexpand: true,
                    set_hexpand: true,

                    set_icon_name: Some(APP_ID),

                    set_title: "Loading",

                    #[watch]
                    set_description: Some(&model.active_stage),

                    #[local_ref]
                    progress_bar -> gtk::ProgressBar {
                        set_halign: gtk::Align::Center,

                        set_width_request: 200
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            progress_bar: gtk::ProgressBar::new(),

            active_stage: String::new()
        };

        let progress_bar = &model.progress_bar;

        let widgets = view_output!();

        unsafe {
            WINDOW = Some(widgets.window.clone());
        }

        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();

            let result = runtime.block_on(async {
                load_app::load_app(&sender).await
            });

            match result {
                Ok(result) => {
                    // dbg!(&result);
    
                    gtk::glib::MainContext::default().spawn(async {
                        let main_app = MainApp::builder()
                            .launch(result)
                            .detach();
    
                        main_app.widget().connect_close_request(|_| {
                            relm4::main_application().quit();
    
                            gtk::glib::Propagation::Proceed
                        });
    
                        unsafe {
                            let window = WINDOW.as_ref()
                                .unwrap_unchecked();
    
                            window.set_hide_on_close(true);
                            window.close();
    
                            main_app.widget()
                                .present();
    
                            MAIN_WINDOW = Some(main_app);
                        }
                    });
                }
    
                Err(err) => sender.input(err)
            }
        });

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            Self::Input::SetActiveStage(text) => self.active_stage = text,
            Self::Input::SetProgress(fraction) => self.progress_bar.set_fraction(fraction),

            Self::Input::DisplayError { title, message } => {
                let window = unsafe {
                    WINDOW.as_ref().unwrap_unchecked()
                };

                let dialog = adw::MessageDialog::new(
                    Some(window),
                    Some(&title),
                    Some(&message)
                );

                dialog.add_response("close", "Close");
                dialog.set_response_appearance("close", adw::ResponseAppearance::Destructive);

                dialog.connect_response(None, |_, _| relm4::main_application().quit());

                dialog.present();
            }
        }
    }
}
