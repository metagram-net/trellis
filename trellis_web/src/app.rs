use serde_json;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod board;
mod clock;
mod note;
mod settings;
mod weather;

// TODO: Make a settings agent that abstracts LocalStorage and backend storage

pub struct App {
    link: ComponentLink<Self>,
    settings: trellis_core::Settings,
    state: State,
    textarea_ref: NodeRef,
    settings_service: Box<dyn Bridge<settings::Settings>>,
}

pub enum Msg {
    FetchSettings,
    ReceiveSettings(trellis_core::Settings),
    EditSettings,
    SaveSettings,
}

enum State {
    Loading,
    Editing,
    Viewing,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: Loading spinner while waiting for settings to sync.
        link.send_message(Msg::FetchSettings);

        Self {
            link: link.clone(),
            settings: trellis_core::Settings::default(),
            textarea_ref: NodeRef::default(),
            state: State::Viewing,
            settings_service: settings::Settings::bridge(link.callback(Msg::ReceiveSettings)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSettings => {
                self.settings_service.send(settings::Request::Load);
                self.state = State::Loading;
                true
            }
            Msg::ReceiveSettings(settings) => {
                // TODO: Merge settings
                self.settings = settings;
                self.state = State::Viewing;
                true
            }
            Msg::EditSettings => {
                self.textarea_ref = NodeRef::default();
                self.state = State::Editing;
                true
            }
            Msg::SaveSettings => {
                let text = self
                    .textarea_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                if let Ok(settings) = serde_json::from_str::<trellis_core::Settings>(&text) {
                    self.settings_service
                        .send(settings::Request::Save(settings.clone()));
                    self.settings = settings;
                    return true;
                }
                // TODO: Don't silently error on serialization issues!
                self.state = State::Editing;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.state {
            State::Loading => self.view_loading(),
            State::Editing => self.view_settings(),
            State::Viewing => self.view_board(),
        }
    }
}

impl App {
    fn view_loading(&self) -> Html {
        html! { <p class="text-xl">{"Loading..."}</p> }
    }

    fn view_settings(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::SaveSettings
        });
        let text =
            serde_json::to_string_pretty(&self.settings).expect("Could not serialize settings");
        let rows = format!(
            "{}",
            // Add an extra row because the last line doesn't have a newline.
            text.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
        );

        html! {
            <form onsubmit=onsubmit>
                <textarea class="w-full" rows=rows ref=self.textarea_ref.clone()>
                    {text}
                </textarea>
                <button type="submit" class="btn btn-gray">{ "Save" }</button>
            </form>
        }
    }

    fn view_board(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::EditSettings);
        html! {
            <>
                <board::Board settings=self.settings.clone() />
                <button type="button" onclick=onclick class="btn btn-gray">{ "Settings" }</button>
            </>
        }
    }
}
