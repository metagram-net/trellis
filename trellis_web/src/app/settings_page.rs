use super::config_form;
use super::settings;
use trellis_core::config;
use yew::prelude::*;

pub struct SettingsPage {
    link: ComponentLink<Self>,
    settings: Option<config::Config>,
    settings_service: Box<dyn Bridge<settings::Settings>>,
}

pub enum Msg {
    Load(config::Config),
    Save(config::Config),
}

impl Component for SettingsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut settings_service = settings::Settings::bridge(link.callback(Msg::Load));
        settings_service.send(settings::Request::Load);
        Self {
            link,
            settings: None,
            settings_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Load(cfg) => {
                self.settings = Some(cfg);
                true
            }
            Msg::Save(cfg) => {
                self.settings_service.send(settings::Request::Save(cfg));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.settings {
            None => html! { <p class="text-xl">{"Loading..."}</p> },
            Some(cfg) => {
                let onsubmit = self.link.callback(Msg::Save);
                html! {
                    <config_form::ConfigForm
                        config=cfg.clone()
                        onsubmit=onsubmit
                    />
                }
            }
        }
    }
}
